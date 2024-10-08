use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::Read;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use bytes::BytesMut;
use clap::Args;

use common::config::Config;
use ed25519_dalek::pkcs8::spki::der::pem::LineEnding;
use ed25519_dalek::pkcs8::{DecodePrivateKey, EncodePrivateKey, EncodePublicKey};
use ed25519_dalek::SigningKey;
use russh::server::Server;
use russh_keys::key::KeyPair;

use common::model::CommonOptions;
use jupiter::context::Context;
use tokio::sync::Mutex;

use crate::git_protocol::ssh::SshServer;

#[derive(Args, Clone, Debug)]
pub struct SshOptions {
    #[clap(flatten)]
    pub common: CommonOptions,

    #[clap(flatten)]
    pub custom: SshCustom,
}

#[derive(Args, Clone, Debug)]
pub struct SshCustom {
    #[arg(long, default_value_t = 2222)]
    ssh_port: u16,

    #[arg(long, value_name = "FILE")]
    ssh_key_path: Option<PathBuf>,

    #[arg(long, value_name = "FILE")]
    ssh_cert_path: Option<PathBuf>,
}

/// start a ssh server
pub async fn start_server(config: Config, command: &SshOptions) {
    // we need to persist the key to prevent key expired after server restart.
    let client_key = load_key(config.ssh.ssh_key_path.clone()).unwrap();
    let client_pubkey = Arc::new(client_key.clone_public_key().unwrap());

    let mut ru_config = russh::server::Config {
        auth_rejection_time: std::time::Duration::from_secs(3),
        auth_rejection_time_initial: Some(std::time::Duration::from_secs(0)),
        // preferred: Preferred {
        //     key: &[russh_keys::key::SSH_RSA],
        //     ..Default::default()
        // },
        ..Default::default()
    };
    ru_config.keys.push(client_key);

    let ru_config = Arc::new(ru_config);

    let SshOptions {
        common: CommonOptions { host, .. },
        custom:
            SshCustom {
                ssh_port,
                ssh_key_path: _,
                ssh_cert_path: _,
            },
    } = command;
    let context = Context::new(config).await;
    let mut ssh_server = SshServer {
        client_pubkey,
        clients: Arc::new(Mutex::new(HashMap::new())),
        id: 0,
        context,
        smart_protocol: None,
        data_combined: BytesMut::new(),
    };
    let server_url = format!("{}:{}", host, ssh_port);
    let addr = SocketAddr::from_str(&server_url).unwrap();
    ssh_server.run_on_address(ru_config, addr).await.unwrap();
}

/// # Loads an SSH keypair.
///
/// This function follows the following steps:
/// 1. It retrieves the root directory for the SSH key from path.
/// 2. It constructs the path to the SSH private key file by joining the root directory with the filename "id_rsa" using PathBuf.
/// 3. It checks if the key file exists. If it doesn't, it generates a new Ed25519 keypair using KeyPair::generate_ed25519.
/// - The generated keypair is then written to the key file.
/// 4. If the key file exists, it reads the keypair from the file.
/// - The keypair is loaded from the file and returned.
///
/// # Returns
///
/// An asynchronous Result containing the loaded SSH keypair if successful, or an error if any of the steps fail.
pub fn load_key(key_root: PathBuf) -> Result<KeyPair> {
    let key_path = PathBuf::from(&key_root).join("id_rsa");

    if !Path::new(&key_root).exists() {
        // create ssh directory if not exists
        create_dir_all(&key_root).unwrap();
    }

    if !key_path.exists() {
        // generate a keypair if not exists
        let keys = KeyPair::generate_ed25519().unwrap();

        if let KeyPair::Ed25519(inner_pair) = &keys {
            // Handle other variants or provide a default behavior
            inner_pair
                .write_pkcs8_pem_file(key_path, LineEnding::CR)
                .unwrap();
            inner_pair
                .verifying_key()
                .write_public_key_pem_file(
                    PathBuf::from(&key_root).join("id_rsa.pub"),
                    LineEnding::CR,
                )
                .unwrap();
        }
        Ok(keys)
    } else {
        // load the keypair from the file
        let mut file = File::open(&key_path)?;
        let mut pem_str = String::new();
        file.read_to_string(&mut pem_str).unwrap();
        let keypair = SigningKey::from_pkcs8_pem(&pem_str)?;
        Ok(KeyPair::Ed25519(keypair))
    }
}
