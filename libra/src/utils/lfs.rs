use std::fs::File;
use std::{fs, io};
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use lazy_static::lazy_static;
use path_abs::{PathInfo, PathOps};
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use sha2::{Digest, Sha256};
use wax::Pattern;
use crate::utils::{path, util};
use crate::utils::path_ext::PathExt;

lazy_static! {
    static ref LFS_PATTERNS: Vec<String> = { // cache
        let attr_path = path::attributes().to_string_or_panic();
        extract_lfs_patterns(&attr_path).unwrap()
    };

    pub static ref LFS_HEADERS: HeaderMap = {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.git-lfs+json"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/vnd.git-lfs+json"));
        headers
    };
}

/// Check if a file is LFS tracked
/// - support Glob pattern matching (TODO: support .gitignore patterns)
/// - only check root attributes file now, should check all attributes files in sub-dirs
/// - absolute path or relative path to workdir
pub fn is_lfs_tracked<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    if LFS_PATTERNS.is_empty() {
        return false;
    }

    let path = util::to_workdir_path(path);
    let glob = wax::any(LFS_PATTERNS.iter().map(|s| s.as_str()).collect::<Vec<_>>()).unwrap();
    glob.is_match(path.to_str().unwrap())
}

const LFS_VERSION: &str = "https://git-lfs.github.com/spec/v1";
/// This is the original & default transfer adapter. All Git LFS clients and servers SHOULD support it.
pub const LFS_TRANSFER_API: &str = "basic";
pub const LFS_HASH_ALGO: &str = "sha256";
const LFS_OID_LEN: usize = 64;
const LFS_POINTER_MAX_SIZE: usize = 300; // bytes

/// Generate lfs pointer file string
/// - return (pointer content, lfs oid)
/// - absolute path
pub fn generate_pointer_file(path: impl AsRef<Path>) -> (String, String) {
    let path = path.as_ref();
    // calc file hash without type
    let oid = calc_lfs_file_hash(path).unwrap();

    let pointer = format!("version {}\noid {}:{}\nsize {}\n",
                          LFS_VERSION, LFS_HASH_ALGO, oid, path.metadata().unwrap().len());
    (pointer, oid)
}

/// Generate LFS Server Url from repo Url.
/// By default, Git LFS will append `.git/info/lfs` to the end of a Git remote url to build the LFS server URL.
/// [doc: server-discovery](https://github.com/git-lfs/git-lfs/blob/main/docs/api/server-discovery.md)
/// - like https://git-server.com/foo/bar.git/info/lfs
/// - support ssh & https & git@ format
pub fn generate_lfs_server_url(mut url: String) -> String {
    if url.ends_with('/') {
        url.pop();
    }
    if !url.ends_with(".git") {
        url.push_str(".git");
    }
    url.push_str("/info/lfs");

    if url.starts_with("git@") {
        // git@git-server.com:foo/bar.git
        url = "https://".to_string() + &url[4..].replace(":", "/");
    } else if url.starts_with("ssh://") {
        // ssh://git-server.com/foo/bar.git
        url = "https://".to_string() + &url[6..];
    }

    url
}

/// Generate LFS cache path, in `.libra/lfs/objects`
pub fn lfs_object_path(oid: &str) -> PathBuf {
    util::storage_path()
        .join("lfs/objects")
        .join(&oid[..2])
        .join(&oid[2..4])
        .join(oid)
}

/// Copy LFS file to `.libra/lfs/objects`
/// - absolute path
pub fn backup_lfs_file<P>(path: P, oid: &str) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let backup_path = lfs_object_path(oid);
    if !backup_path.exists() {
        fs::create_dir_all(backup_path.parent().unwrap())?;
        fs::copy(path, backup_path)?;
    }
    Ok(())
}

/// SHA256 without type
/// TODO: performance optimization, 200MB 4s now, slower than `sha256sum`
pub fn calc_lfs_file_hash<P>(path: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let mut hash = Sha256::new();
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 65536];
    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hash.update(&buffer[..n]);
    }
    let file_hash = hex::encode(hash.finalize());
    Ok(file_hash)
}

/// Check if `data` is an LFS pointer, return `oid`
pub fn parse_pointer_data(data: &[u8]) -> Option<(String, u64)> {
    if data.len() > LFS_POINTER_MAX_SIZE {
        return None;
    }
    // Start with format `version ...`
    if let Some(data) = data.strip_prefix(format!("version {}\noid {}:", LFS_VERSION, LFS_HASH_ALGO).as_bytes()) {
        if data[LFS_OID_LEN] == b'\n' {
            // check `oid` length
            let oid = String::from_utf8(data[..LFS_OID_LEN].to_vec()).unwrap();
            if let Some(data) = data.strip_prefix(format!("{}\nsize ", oid).as_bytes()) {
                let data = String::from_utf8(data[..].to_vec()).unwrap();
                if let Ok(size) = data.trim_end().parse::<u64>() {
                    return Some((oid, size));
                }
            }
        }
    }
    None
}

/// Extract LFS patterns from `.libra_attributes` file
pub fn extract_lfs_patterns(file_path: &str) -> io::Result<Vec<String>> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Ok(Vec::new());
    }
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // ' ' needs '\' before it to be escaped
    let re = Regex::new(r"^\s*(([^\s#\\]|\\ )+)").unwrap();

    let mut patterns = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if !line.contains("filter=lfs") {
            continue;
        }
        if let Some(cap) = re.captures(&line) {
            if let Some(pattern) = cap.get(1) {
                let pattern = pattern.as_str().replace(r"\ ", " ");
                patterns.push(pattern);
            }
        }
    }

    Ok(patterns)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_pointer_file() {
        let path = Path::new("../tests/data/packs/git-2d187177923cd618a75da6c6db45bb89d92bd504.pack");
        let (pointer, _oid) = generate_pointer_file(path);
        print!("{}", pointer);
    }

    #[test]
    fn test_is_pointer_file() {
        let data = b"version https://git-lfs.github.com/spec/v1\noid sha256:1234567890abcdef\nsize 1234\n";
        assert!(parse_pointer_data(data).is_some());
    }

    #[test]
    fn test_gen_lfs_server_url() {
        const LFS_SERVER_URL: &str = "https://github.com/web3infra-foundation/mega.git/info/lfs";
        let url = "https://github.com/web3infra-foundation/mega".to_owned();
        assert_eq!(generate_lfs_server_url(url), LFS_SERVER_URL);

        let url = "https://github.com/web3infra-foundation/mega.git".to_owned();
        assert_eq!(generate_lfs_server_url(url), LFS_SERVER_URL);

        let url = "git@github.com:web3infra-foundation/mega.git".to_owned();
        assert_eq!(generate_lfs_server_url(url), LFS_SERVER_URL);

        let url = "ssh://github.com/web3infra-foundation/mega.git".to_owned();
        assert_eq!(generate_lfs_server_url(url), LFS_SERVER_URL);
    }

    #[test]
    fn test_parse_pointer_data() {
        let data = r#"version https://git-lfs.github.com/spec/v1
oid sha256:4859402c258b836d02e955d1090e29f586e58b2040504d68afec3d8d43757bba
size 10
"#;
        let res = parse_pointer_data(data.as_bytes()).unwrap();
        println!("{:?}", res);
        assert_eq!(res.0, "4859402c258b836d02e955d1090e29f586e58b2040504d68afec3d8d43757bba");
        assert_eq!(res.1, 10);
    }
}