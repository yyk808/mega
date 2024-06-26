use std::sync::Arc;

use async_trait::async_trait;
use common::config::StorageConfig;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
use sea_orm::{PaginatorTrait, QueryOrder};

use callisto::{git_blob, git_commit, git_repo, git_tag, git_tree, import_refs, raw_blob};
use common::errors::MegaError;
use mercury::internal::object::GitObjectModel;
use mercury::internal::pack::entry::Entry;
use venus::import_repo::import_refs::RefCommand;
use venus::import_repo::import_refs::Refs;
use venus::import_repo::repo::Repo;

use crate::{
    raw_storage::{self, RawStorage},
    storage::GitStorageProvider,
};

use super::batch_save_model;

#[derive(Clone)]
pub struct GitDbStorage {
    pub raw_storage: Arc<dyn RawStorage>,
    pub connection: Arc<DatabaseConnection>,
    pub raw_obj_threshold: usize,
}

#[async_trait]
impl GitStorageProvider for GitDbStorage {
    async fn save_ref(&self, repo: &Repo, refs: &RefCommand) -> Result<(), MegaError> {
        let mut model: import_refs::Model = refs.clone().into();
        model.repo_id = repo.repo_id;
        let a_model = model.into_active_model();
        import_refs::Entity::insert(a_model)
            .exec(self.get_connection())
            .await
            .unwrap();
        Ok(())
    }

    async fn remove_ref(&self, repo: &Repo, refs: &RefCommand) -> Result<(), MegaError> {
        import_refs::Entity::delete_many()
            .filter(import_refs::Column::RepoId.eq(repo.repo_id))
            .filter(import_refs::Column::RefName.eq(refs.ref_name.clone()))
            .exec(self.get_connection())
            .await?;
        Ok(())
    }

    async fn get_ref(&self, repo: &Repo) -> Result<Vec<Refs>, MegaError> {
        let result = import_refs::Entity::find()
            .filter(import_refs::Column::RepoId.eq(repo.repo_id))
            .order_by_asc(import_refs::Column::RefName)
            .all(self.get_connection())
            .await?;
        let res: Vec<Refs> = result.into_iter().map(|x| x.into()).collect();
        Ok(res)
    }

    async fn update_ref(&self, repo: &Repo, ref_name: &str, new_id: &str) -> Result<(), MegaError> {
        let ref_data: import_refs::Model = import_refs::Entity::find()
            .filter(import_refs::Column::RepoId.eq(repo.repo_id))
            .filter(import_refs::Column::RefName.eq(ref_name))
            .one(self.get_connection())
            .await
            .unwrap()
            .unwrap();
        let mut ref_data: import_refs::ActiveModel = ref_data.into();
        ref_data.ref_git_id = Set(new_id.to_string());
        ref_data.updated_at = Set(chrono::Utc::now().naive_utc());
        ref_data.update(self.get_connection()).await.unwrap();
        Ok(())
    }
}

impl GitDbStorage {
    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }

    pub async fn new(connection: Arc<DatabaseConnection>, config: StorageConfig) -> Self {
        GitDbStorage {
            connection,
            raw_storage: raw_storage::init(config.raw_obj_storage_type, config.raw_obj_local_path)
                .await,
            raw_obj_threshold: config.big_obj_threshold,
        }
    }

    pub fn mock() -> Self {
        GitDbStorage {
            connection: Arc::new(DatabaseConnection::default()),
            raw_storage: raw_storage::mock(),
            raw_obj_threshold: 1024,
        }
    }

    pub async fn get_default_ref(&self, repo: &Repo) -> Result<Option<Refs>, MegaError> {
        let result = import_refs::Entity::find()
            .filter(import_refs::Column::RepoId.eq(repo.repo_id))
            .filter(import_refs::Column::DefaultBranch.eq(true))
            .one(self.get_connection())
            .await?;
        if let Some(model) = result {
            let refs: Refs = model.into();
            Ok(Some(refs))
        } else {
            Ok(None)
        }
    }

    pub async fn default_branch_exist(&self, repo: &Repo) -> Result<bool, MegaError> {
        let result = import_refs::Entity::find()
            .filter(import_refs::Column::RepoId.eq(repo.repo_id))
            .filter(import_refs::Column::DefaultBranch.eq(true))
            .count(self.get_connection())
            .await?;
        Ok(result > 0)
    }

    pub async fn save_entry(&self, repo: &Repo, entry_list: Vec<Entry>) -> Result<(), MegaError> {
        let mut commits = Vec::new();
        let mut trees = Vec::new();
        let mut blobs = Vec::new();
        let mut raw_blobs = Vec::new();
        let mut tags = Vec::new();

        for entry in entry_list {
            let raw_obj = entry.process_entry();
            let model = raw_obj.convert_to_git_model();
            match model {
                GitObjectModel::Commit(mut commit) => {
                    commit.repo_id = repo.repo_id;
                    commits.push(commit.into_active_model())
                }
                GitObjectModel::Tree(mut tree) => {
                    tree.repo_id = repo.repo_id;
                    trees.push(tree.clone().into_active_model());
                }
                GitObjectModel::Blob(mut blob, raw) => {
                    blob.repo_id = repo.repo_id;
                    blobs.push(blob.clone().into_active_model());
                    raw_blobs.push(raw.into_active_model());
                }
                GitObjectModel::Tag(mut tag) => {
                    tag.repo_id = repo.repo_id;
                    tags.push(tag.into_active_model())
                }
            }
        }

        batch_save_model(self.get_connection(), commits)
            .await
            .unwrap();
        batch_save_model(self.get_connection(), trees)
            .await
            .unwrap();
        batch_save_model(self.get_connection(), blobs)
            .await
            .unwrap();
        batch_save_model(self.get_connection(), raw_blobs)
            .await
            .unwrap();
        batch_save_model(self.get_connection(), tags).await.unwrap();
        Ok(())
    }

    pub async fn find_git_repo(
        &self,
        repo_path: &str,
    ) -> Result<Option<git_repo::Model>, MegaError> {
        let result = git_repo::Entity::find()
            .filter(git_repo::Column::RepoPath.eq(repo_path))
            .one(self.get_connection())
            .await?;
        Ok(result)
    }

    pub async fn save_git_repo(&self, repo: Repo) -> Result<(), MegaError> {
        let model: git_repo::Model = repo.into();
        let a_model = model.into_active_model();
        git_repo::Entity::insert(a_model)
            .exec(self.get_connection())
            .await
            .unwrap();
        Ok(())
    }

    // #[allow(unused)]
    // pub async fn update_git_repo(&self, repo: Repo) -> Result<(), MegaError> {
    //     let git_repo = git_repo::Entity::find_by_id(repo.repo_id)
    //         .one(self.get_connection())
    //         .await
    //         .unwrap();
    //     let git_repo: git_repo::ActiveModel = git_repo.unwrap().into();
    //     git_repo.update(self.get_connection()).await.unwrap();
    //     Ok(())
    // }

    pub async fn get_commit_by_hash(
        &self,
        repo: &Repo,
        hash: &str,
    ) -> Result<Option<git_commit::Model>, MegaError> {
        Ok(git_commit::Entity::find()
            .filter(git_commit::Column::RepoId.eq(repo.repo_id))
            .filter(git_commit::Column::CommitId.eq(hash))
            .one(self.get_connection())
            .await
            .unwrap())
    }

    pub async fn get_commits_by_repo_id(
        &self,
        repo: &Repo,
    ) -> Result<Vec<git_commit::Model>, MegaError> {
        Ok(git_commit::Entity::find()
            .filter(git_commit::Column::RepoId.eq(repo.repo_id))
            .all(self.get_connection())
            .await
            .unwrap())
    }

    pub async fn get_trees_by_repo_id(
        &self,
        repo: &Repo,
    ) -> Result<Vec<git_tree::Model>, MegaError> {
        Ok(git_tree::Entity::find()
            .filter(git_tree::Column::RepoId.eq(repo.repo_id))
            .all(self.get_connection())
            .await
            .unwrap())
    }

    pub async fn get_trees_by_hashes(
        &self,
        repo: &Repo,
        hashes: Vec<String>,
    ) -> Result<Vec<git_tree::Model>, MegaError> {
        Ok(git_tree::Entity::find()
            .filter(git_tree::Column::RepoId.eq(repo.repo_id))
            .filter(git_tree::Column::TreeId.is_in(hashes))
            .all(self.get_connection())
            .await
            .unwrap())
    }

    pub async fn get_tree_by_hash(
        &self,
        repo: &Repo,
        hash: &str,
    ) -> Result<Option<git_tree::Model>, MegaError> {
        Ok(git_tree::Entity::find()
            .filter(git_tree::Column::RepoId.eq(repo.repo_id))
            .filter(git_tree::Column::TreeId.eq(hash))
            .one(self.get_connection())
            .await
            .unwrap())
    }

    pub async fn get_blobs_by_repo_id(
        &self,
        repo: &Repo,
    ) -> Result<Vec<git_blob::Model>, MegaError> {
        Ok(git_blob::Entity::find()
            .filter(git_blob::Column::RepoId.eq(repo.repo_id))
            .all(self.get_connection())
            .await
            .unwrap())
    }

    pub async fn get_tags_by_repo_id(&self, repo: &Repo) -> Result<Vec<git_tag::Model>, MegaError> {
        Ok(git_tag::Entity::find()
            .filter(git_tag::Column::RepoId.eq(repo.repo_id))
            .all(self.get_connection())
            .await
            .unwrap())
    }

    pub async fn get_obj_count_by_repo_id(&self, repo: &Repo) -> usize {
        let c_count = git_commit::Entity::find()
            .filter(git_commit::Column::RepoId.eq(repo.repo_id))
            .count(self.get_connection())
            .await
            .unwrap();

        let t_count = git_tree::Entity::find()
            .filter(git_tree::Column::RepoId.eq(repo.repo_id))
            .count(self.get_connection())
            .await
            .unwrap();

        let bids: Vec<String> = self
            .get_blobs_by_repo_id(repo)
            .await
            .unwrap()
            .into_iter()
            .map(|b| b.blob_id)
            .collect();

        let b_count = raw_blob::Entity::find()
            .filter(raw_blob::Column::Sha1.is_in(bids))
            .count(self.get_connection())
            .await
            .unwrap();

        let tag_count = git_tag::Entity::find()
            .filter(git_tag::Column::RepoId.eq(repo.repo_id))
            .count(self.get_connection())
            .await
            .unwrap();

        (c_count + t_count + b_count + tag_count)
            .try_into()
            .unwrap()
    }
}
