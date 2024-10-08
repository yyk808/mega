use std::path::Path;
use async_static::async_static;
use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use url::Url;
use ceres::lfs::lfs_structs::{BatchRequest, Representation, RequestVars};
use mercury::internal::object::types::ObjectType;
use mercury::internal::pack::entry::Entry;
use crate::internal::config::Config;
use crate::internal::protocol::https_client::BasicAuth;
use crate::internal::protocol::ProtocolClient;
use crate::utils::lfs;

async_static! {
    pub static ref LFS_CLIENT: LFSClient = LFSClient::new().await;
}

pub struct LFSClient {
    pub url: Url,
    pub client: Client,
}

/// see [successful-responses](https://github.com/git-lfs/git-lfs/blob/main/docs/api/batch.md#successful-responses)
#[derive(Serialize, Deserialize)]
struct LfsBatchResponse {
    transfer: Option<String>,
    objects: Vec<Representation>,
    hash_algo: Option<String>,
}

impl ProtocolClient for LFSClient {
    /// Construct LFSClient from a given Repo URL.
    fn from_url(repo_url: &Url) -> Self {
        let lfs_server = Url::parse(&lfs::generate_lfs_server_url(repo_url.to_string())).unwrap();
        let client = Client::builder()
            .http1_only()
            .default_headers(lfs::LFS_HEADERS.clone())
            .build()
            .unwrap();
        Self {
            url: lfs_server.join("/objects/batch").unwrap(),
            client,
        }
    }
}

impl LFSClient {
    /// Construct LFSClient from current remote URL.
    pub async fn new() -> Self {
        let url = Config::get_current_remote_url().await;
        match url {
            Some(url) => LFSClient::from_url(&Url::parse(&url).unwrap()),
            None => panic!("fatal: current remote url not found"),
        }
    }

    /// push LFS objects to remote server
    pub async fn push_objects<'a, I>(&self, objs: I, auth: Option<BasicAuth>)
    where
        I: IntoIterator<Item = &'a Entry>
    {
        // filter pointer file within blobs
        let mut lfs_oids = Vec::new();
        for blob in objs.into_iter().filter(|e| e.obj_type == ObjectType::Blob) {
            let oid = lfs::parse_pointer_data(&blob.data);
            if let Some(oid) = oid {
                lfs_oids.push(oid);
            }
        }

        let mut lfs_objs = Vec::new();
        for (oid, _) in &lfs_oids {
            let path = lfs::lfs_object_path(oid);
            if !path.exists() {
                eprintln!("fatal: LFS object not found: {}", oid);
                return;
            }
            let size = path.metadata().unwrap().len() as i64;
            lfs_objs.push(RequestVars {
                oid: oid.to_owned(),
                size,
                ..Default::default()
            })
        }

        let batch_request = BatchRequest {
            operation: "upload".to_string(),
            transfers: vec![lfs::LFS_TRANSFER_API.to_string()],
            objects: lfs_objs,
            hash_algo: lfs::LFS_HASH_ALGO.to_string(),
            enable_split: None,
        };

        let mut request = self.client.post(self.url.clone()).json(&batch_request);
        if let Some(auth) = auth {
            request = request.basic_auth(auth.username, Some(auth.password));
        }

        let response = request.send().await.unwrap();

        let resp = response.json::<LfsBatchResponse>().await.unwrap();
        tracing::debug!("LFS push response:\n {:#?}", serde_json::to_value(&resp).unwrap());

        // TODO: parallel upload
        for obj in resp.objects {
            self.upload_object(obj).await;
        }
        println!("LFS objects push completed.");
    }

    /// upload (PUT) one LFS file to remote server
    async fn upload_object(&self, object: Representation) {
        if let Some(err) = object.error {
            eprintln!("fatal: LFS upload failed. Code: {}, Message: {}", err.code, err.message);
            return;
        }

        if let Some(actions) = object.actions {
            let upload_link = actions.get("upload");
            if upload_link.is_none() {
                eprintln!("fatal: LFS upload failed. No upload action found");
                return;
            }

            let link = upload_link.unwrap();
            let mut request = self.client.put(link.href.clone());
            for (k, v) in &link.header {
                request = request.header(k, v);
            }

            let file_path = lfs::lfs_object_path(&object.oid);
            let file = tokio::fs::File::open(file_path).await.unwrap();
            println!("Uploading LFS file: {}", object.oid);
            let resp = request
                .body(reqwest::Body::wrap_stream(tokio_util::io::ReaderStream::new(file)))
                .send()
                .await
                .unwrap();
            if !resp.status().is_success() {
                eprintln!("fatal: LFS upload failed. Status: {}, Message: {}", resp.status(), resp.text().await.unwrap());
                return;
            }
            println!("Uploaded.");
        } else {
            tracing::debug!("LFS file {} already exists on remote server", object.oid);
        }
    }

    /// download (GET) one LFS file from remote server
    pub async fn download_object(&self, oid: &str, size: u64, path: impl AsRef<Path>) {
        let batch_request = BatchRequest {
            operation: "download".to_string(),
            transfers: vec![lfs::LFS_TRANSFER_API.to_string()],
            objects: vec![RequestVars {
                oid: oid.to_owned(),
                size: size as i64,
                ..Default::default()
            }],
            hash_algo: lfs::LFS_HASH_ALGO.to_string(),
            enable_split: None,
        };

        let request = self.client.post(self.url.clone()).json(&batch_request);
        let response = request.send().await.unwrap();

        let resp = response.json::<LfsBatchResponse>().await.unwrap();
        tracing::debug!("LFS download response:\n {:#?}", serde_json::to_value(&resp).unwrap());

        let link = resp.objects[0].actions.as_ref().unwrap().get("download").unwrap();

        let mut request = self.client.get(link.href.clone());
        for (k, v) in &link.header {
            request = request.header(k, v);
        }

        let response = request.send().await.unwrap();

        if !response.status().is_success() {
            eprintln!("fatal: LFS download failed. Status: {}, Message: {}", response.status(), response.text().await.unwrap());
            return;
        }

        println!("Downloading LFS file: {}", oid);
        let mut file = tokio::fs::File::create(path).await.unwrap();
        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.unwrap();
            file.write_all(&chunk).await.unwrap();
        }
        println!("Downloaded."); // TODO: checksum
    }
}