use std::sync::mpsc;

use async_trait::async_trait;
use bytes::Bytes;

use callisto::raw_blob;
use common::errors::MegaError;
use jupiter::{
    context::Context,
    storage::{batch_query_by_columns, GitStorageProvider},
};
use mercury::internal::pack::encode::PackEncoder;
use mercury::{
    errors::GitError,
    internal::{
        object::{blob::Blob, commit::Commit, tag::Tag, tree::Tree},
        pack::entry::Entry,
    },
};
use venus::import_repo::{
    import_refs::{CommandType, RefCommand, Refs},
    repo::Repo,
};

use crate::pack::handler::PackHandler;

pub struct ImportRepo {
    pub context: Context,
    pub repo: Repo,
}

#[async_trait]
impl PackHandler for ImportRepo {
    async fn head_hash(&self) -> (String, Vec<Refs>) {
        let refs: Vec<Refs> = self
            .context
            .services
            .git_db_storage
            .get_ref(&self.repo)
            .await
            .unwrap();

        self.find_head_hash(refs)
    }

    async fn unpack(&self, pack_file: Bytes) -> Result<(), GitError> {
        let receiver = self.pack_decoder(&self.context.config.monorepo, pack_file).unwrap();

        let storage = self.context.services.git_db_storage.clone();
        let mut entry_list = Vec::new();
        for entry in receiver {
            entry_list.push(entry);
            if entry_list.len() >= 1000 {
                storage.save_entry(&self.repo, entry_list).await.unwrap();
                entry_list = Vec::new();
            }
        }
        storage.save_entry(&self.repo, entry_list).await.unwrap();
        Ok(())
    }

    async fn full_pack(&self) -> Result<Vec<u8>, GitError> {
        let (sender, receiver) = mpsc::channel();

        let storage = self.context.services.git_db_storage.clone();
        let total = storage.get_obj_count_by_repo_id(&self.repo).await;
        let mut encoder = PackEncoder::new(total, 0);

        for m in storage
            .get_commits_by_repo_id(&self.repo)
            .await
            .unwrap()
            .into_iter()
        {
            let c: Commit = m.into();
            let entry: Entry = c.into();
            sender.send(entry).unwrap();
        }

        for m in storage
            .get_trees_by_repo_id(&self.repo)
            .await
            .unwrap()
            .into_iter()
        {
            let c: Tree = m.into();
            let entry: Entry = c.into();
            sender.send(entry).unwrap();
        }

        let bids: Vec<String> = storage
            .get_blobs_by_repo_id(&self.repo)
            .await
            .unwrap()
            .into_iter()
            .map(|b| b.blob_id)
            .collect();

        let raw_blobs = batch_query_by_columns::<raw_blob::Entity, raw_blob::Column>(
            storage.get_connection(),
            raw_blob::Column::Sha1,
            bids,
            None,
            None,
        )
        .await
        .unwrap();

        for m in raw_blobs {
            // todo handle storage type
            let c: Blob = m.into();
            let entry: Entry = c.into();
            sender.send(entry).unwrap();
        }

        for m in storage
            .get_tags_by_repo_id(&self.repo)
            .await
            .unwrap()
            .into_iter()
        {
            let c: Tag = m.into();
            let entry: Entry = c.into();
            sender.send(entry).unwrap();
        }
        drop(sender);
        let data = encoder.encode(receiver).unwrap();

        Ok(data)
    }

    async fn incremental_pack(
        &self,
        _want: Vec<String>,
        _have: Vec<String>,
    ) -> Result<Vec<u8>, GitError> {
        unimplemented!()
    }

    async fn get_trees_by_hashes(&self, hashes: Vec<String>) -> Result<Vec<Tree>, MegaError> {
        Ok(self
            .context
            .services
            .git_db_storage
            .get_trees_by_hashes(&self.repo, hashes)
            .await
            .unwrap()
            .into_iter()
            .map(|x| x.into())
            .collect())
    }

    async fn get_blobs_by_hashes(
        &self,
        hashes: Vec<String>,
    ) -> Result<Vec<raw_blob::Model>, MegaError> {
        self.context
            .services
            .mega_storage
            .get_raw_blobs_by_hashes(hashes)
            .await
    }

    async fn update_refs(&self, refs: &RefCommand) -> Result<(), GitError> {
        let storage = self.context.services.git_db_storage.clone();
        match refs.command_type {
            CommandType::Create => {
                storage.save_ref(&self.repo, refs).await.unwrap();
            }
            CommandType::Delete => storage.remove_ref(&self.repo, refs).await.unwrap(),
            CommandType::Update => {
                storage
                    .update_ref(&self.repo, &refs.ref_name, &refs.new_id)
                    .await
                    .unwrap();
            }
        }
        Ok(())
    }

    async fn check_commit_exist(&self, hash: &str) -> bool {
        self.context
            .services
            .git_db_storage
            .get_commit_by_hash(&self.repo, hash)
            .await
            .unwrap()
            .is_some()
    }

    async fn check_default_branch(&self) -> bool {
        let storage = self.context.services.git_db_storage.clone();
        storage.default_branch_exist(&self.repo).await.unwrap()
    }
}
