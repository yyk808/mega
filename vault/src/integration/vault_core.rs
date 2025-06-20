use std::sync::{Arc, RwLock};

use crate::integration::jupiter_backend::JupiterBackend;
use common::errors::MegaError;
use jupiter::context::Context;

use rusty_vault::{
    core::Core,
    logical::{Operation, Request, Response},
    storage::{barrier_aes_gcm, Backend},
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tracing::log;

const CORE_KEY_FILE: &str = "core_key.json"; // where the core key is stored, like `root_token`

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CoreKey {
    secret_shares: Vec<Vec<u8>>,
    root_token: String,
}

#[derive(Clone)]
pub struct VaultCore {
    core: Arc<RwLock<Core>>,
    key: CoreKey,
}

impl VaultCore {
    pub fn new(ctx: Context) -> Self {
        let dir = common::config::mega_base().join("vault");
        let key_path = dir.join(CORE_KEY_FILE);

        std::fs::create_dir_all(&dir).expect("Failed to create vault directory");

        let backend: Arc<dyn Backend> = Arc::new(JupiterBackend::new(ctx));
        let barrier = barrier_aes_gcm::AESGCMBarrier::new(Arc::clone(&backend));
        let seal_config = rusty_vault::core::SealConfig {
            secret_shares: 10,
            secret_threshold: 5,
        };

        let core = Core {
            physical: backend,
            barrier: Arc::new(barrier),
            ..Default::default()
        };
        let core = Arc::new(RwLock::new(core));

        let key = {
            let mut managed_core = core.write().unwrap();
            managed_core
                .config(core.clone(), None)
                .expect("Failed to configure vault core");

            let core_key = if !managed_core
                .inited()
                .expect("Failed to check if vault is initialized")
            {
                let result = managed_core
                    .init(&seal_config)
                    .expect("Failed to initialize vault");
                let core_key = CoreKey {
                    secret_shares: Vec::from(&result.secret_shares[..]),
                    root_token: result.root_token,
                };
                let file = std::fs::File::create(key_path).unwrap();
                serde_json::to_writer_pretty(file, &core_key).unwrap();

                core_key
            } else {
                let key_data =
                    std::fs::read(&key_path).expect("Failed to read vault core key file");
                serde_json::from_slice::<CoreKey>(&key_data)
                    .expect("Failed to deserialize core key")
            };

            for i in 0..seal_config.secret_threshold {
                let key = &core_key.secret_shares[i as usize];
                let unseal = managed_core.unseal(key);
                assert!(unseal.is_ok());
            }

            log::debug!(
                "Vault core initialized with root token: {}",
                core_key.root_token
            );

            core_key
        };

        Self { core, key }
    }

    pub fn token(&self) -> &str {
        &self.key.root_token
    }

    pub(crate) fn read_api(&self, path: impl AsRef<str>) -> Result<Option<Response>, MegaError> {
        let mut req = Request::new(path.as_ref());
        req.operation = Operation::Read;
        req.client_token = self.token().to_string();
        let guard = self.core.read().unwrap();
        guard
            .handle_request(&mut req)
            .map_err(|_| MegaError::with_message("Failed to read from vault API"))
    }

    pub(crate) fn write_api(
        &self,
        path: impl AsRef<str>,
        data: Option<Map<String, Value>>,
    ) -> Result<Option<Response>, MegaError> {
        let mut req = Request::new(path.as_ref());
        req.operation = Operation::Write;
        req.client_token = self.token().to_string();
        req.body = data;
        let guard = self.core.read().unwrap();
        guard
            .handle_request(&mut req)
            .map_err(|_| MegaError::with_message("Failed to write to vault API"))
    }

    pub(crate) fn delete_api(&self, path: impl AsRef<str>) -> Result<Option<Response>, MegaError> {
        let mut req = Request::new(path.as_ref());
        req.operation = Operation::Delete;
        req.client_token = self.token().to_string();
        let guard = self.core.read().unwrap();
        guard
            .handle_request(&mut req)
            .map_err(|_| MegaError::with_message("Failed to delete from vault API"))
    }

    pub fn write_secret(
        &self,
        name: &str,
        data: Option<Map<String, Value>>,
    ) -> Result<(), MegaError> {
        self.write_api(&format!("secret/{}", name), data)
            .map_err(|_| MegaError::with_message(format!("Failed to write secret: {}", name)))?;
        Ok(())
    }

    pub fn read_secret(&self, name: &str) -> Result<Option<Map<String, Value>>, MegaError> {
        let resp = self
            .read_api(&format!("secret/{}", name))
            .map_err(|_| MegaError::with_message(format!("Failed to read secret: {}", name)))?;

        Ok(resp.map(|r| r.data).flatten())
    }

    pub fn delete_secret(&self, name: &str) -> Result<(), MegaError> {
        self.delete_api(&format!("secret/{}", name))
            .map_err(|_| MegaError::with_message(format!("Failed to delete secret: {}", name)))?;
        Ok(())
    }
}
