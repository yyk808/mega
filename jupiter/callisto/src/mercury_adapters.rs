//! Implements From trait for converting mercury models to callisto models

impl From<mercury::internal::model::sea_models::git_commit::Model> for crate::git_commit::Model {
    fn from(model: mercury::internal::model::sea_models::git_commit::Model) -> Self {
        Self {
            id: model.id,
            repo_id: model.repo_id as i64, // Convert i32 to i64
            commit_id: model.commit_id,
            tree: model.tree,
            parents_id: serde_json::from_str(&model.parents_id)
                .unwrap_or(serde_json::Value::Array(vec![])),
            author: model.author,
            committer: model.committer,
            content: model.content,
            created_at: model.created_at, // Keep as NaiveDateTime to match callisto
        }
    }
}

impl From<mercury::internal::model::sea_models::git_commit::ActiveModel>
    for crate::git_commit::ActiveModel
{
    fn from(model: mercury::internal::model::sea_models::git_commit::ActiveModel) -> Self {
        Self {
            id: model.id,
            repo_id: match model.repo_id {
                sea_orm::ActiveValue::Set(val) => sea_orm::ActiveValue::Set(val as i64), // Convert i32 to i64
                sea_orm::ActiveValue::Unchanged(val) => sea_orm::ActiveValue::Unchanged(val as i64), // Convert i32 to i64
                sea_orm::ActiveValue::NotSet => sea_orm::ActiveValue::NotSet,
            },
            commit_id: model.commit_id,
            tree: model.tree,
            parents_id: match model.parents_id {
                sea_orm::ActiveValue::Set(val) => sea_orm::ActiveValue::Set(
                    serde_json::from_str(&val).unwrap_or(serde_json::Value::Array(vec![])),
                ),
                sea_orm::ActiveValue::Unchanged(val) => sea_orm::ActiveValue::Unchanged(
                    serde_json::from_str(&val).unwrap_or(serde_json::Value::Array(vec![])),
                ),
                sea_orm::ActiveValue::NotSet => sea_orm::ActiveValue::NotSet,
            },
            author: model.author,
            committer: model.committer,
            content: model.content,
            created_at: match model.created_at {
                sea_orm::ActiveValue::Set(val) => sea_orm::ActiveValue::Set(val),
                sea_orm::ActiveValue::Unchanged(val) => sea_orm::ActiveValue::Unchanged(val),
                sea_orm::ActiveValue::NotSet => sea_orm::ActiveValue::NotSet,
            },
        }
    }
}

impl From<mercury::internal::model::sea_models::mega_commit::Model> for crate::mega_commit::Model {
    fn from(model: mercury::internal::model::sea_models::mega_commit::Model) -> Self {
        Self {
            id: model.id,
            commit_id: model.commit_id,
            tree: model.tree,
            parents_id: serde_json::from_str(&model.parents_id)
                .unwrap_or(serde_json::Value::Array(vec![])),
            author: model.author,
            committer: model.committer,
            content: model.content,
            created_at: model.created_at, // Keep as NaiveDateTime to match callisto
        }
    }
}

impl From<mercury::internal::model::sea_models::mega_commit::ActiveModel>
    for crate::mega_commit::ActiveModel
{
    fn from(model: mercury::internal::model::sea_models::mega_commit::ActiveModel) -> Self {
        Self {
            id: model.id,
            commit_id: model.commit_id,
            tree: model.tree,
            parents_id: match model.parents_id {
                sea_orm::ActiveValue::Set(val) => sea_orm::ActiveValue::Set(
                    serde_json::from_str(&val).unwrap_or(serde_json::Value::Array(vec![])),
                ),
                sea_orm::ActiveValue::Unchanged(val) => sea_orm::ActiveValue::Unchanged(
                    serde_json::from_str(&val).unwrap_or(serde_json::Value::Array(vec![])),
                ),
                sea_orm::ActiveValue::NotSet => sea_orm::ActiveValue::NotSet,
            },
            author: model.author,
            committer: model.committer,
            content: model.content,
            created_at: match model.created_at {
                sea_orm::ActiveValue::Set(val) => sea_orm::ActiveValue::Set(val),
                sea_orm::ActiveValue::Unchanged(val) => sea_orm::ActiveValue::Unchanged(val),
                sea_orm::ActiveValue::NotSet => sea_orm::ActiveValue::NotSet,
            },
        }
    }
}

// -------------------------
// mega_tree adapters
// -------------------------
impl From<mercury::internal::model::sea_models::mega_tree::Model> for crate::mega_tree::Model {
    fn from(model: mercury::internal::model::sea_models::mega_tree::Model) -> Self {
        Self {
            id: model.id,
            tree_id: model.tree_id,
            sub_trees: model.sub_trees,
            size: model.size,
            commit_id: model.commit_id,
            created_at: model.created_at,
        }
    }
}

impl From<mercury::internal::model::sea_models::mega_tree::ActiveModel>
    for crate::mega_tree::ActiveModel
{
    fn from(model: mercury::internal::model::sea_models::mega_tree::ActiveModel) -> Self {
        Self {
            id: model.id,
            tree_id: model.tree_id,
            sub_trees: model.sub_trees,
            size: model.size,
            commit_id: model.commit_id,
            created_at: model.created_at,
        }
    }
}

// -------------------------
// mega_blob adapters
// -------------------------
impl From<mercury::internal::model::sea_models::mega_blob::Model> for crate::mega_blob::Model {
    fn from(model: mercury::internal::model::sea_models::mega_blob::Model) -> Self {
        Self {
            id: model.id,
            blob_id: model.blob_id,
            commit_id: model.commit_id,
            name: model.name,
            size: model.size,
            created_at: model.created_at,
        }
    }
}

impl From<mercury::internal::model::sea_models::mega_blob::ActiveModel>
    for crate::mega_blob::ActiveModel
{
    fn from(model: mercury::internal::model::sea_models::mega_blob::ActiveModel) -> Self {
        Self {
            id: model.id,
            blob_id: model.blob_id,
            commit_id: model.commit_id,
            name: model.name,
            size: model.size,
            created_at: model.created_at,
        }
    }
}

// -------------------------
// raw_blob adapters (with StorageTypeEnum mapping)
// -------------------------
impl From<mercury::internal::model::sea_models::raw_blob::Model> for crate::raw_blob::Model {
    fn from(model: mercury::internal::model::sea_models::raw_blob::Model) -> Self {
        let storage_type = match model.storage_type.as_str() {
            "database" | "Database" => crate::sea_orm_active_enums::StorageTypeEnum::Database,
            "local_fs" | "LocalFs" | "fs" => crate::sea_orm_active_enums::StorageTypeEnum::LocalFs,
            "aws_s3" | "s3" | "S3" => crate::sea_orm_active_enums::StorageTypeEnum::AwsS3,
            _ => crate::sea_orm_active_enums::StorageTypeEnum::Database,
        };
        Self {
            id: model.id,
            sha1: model.sha1,
            content: model.content,
            file_type: model.file_type,
            storage_type,
            data: model.data,
            local_path: model.local_path,
            remote_url: model.remote_url,
            created_at: model.created_at,
        }
    }
}

impl From<mercury::internal::model::sea_models::raw_blob::ActiveModel>
    for crate::raw_blob::ActiveModel
{
    fn from(model: mercury::internal::model::sea_models::raw_blob::ActiveModel) -> Self {
        let map_storage = |s: String| match s.as_str() {
            "database" | "Database" => crate::sea_orm_active_enums::StorageTypeEnum::Database,
            "local_fs" | "LocalFs" | "fs" => crate::sea_orm_active_enums::StorageTypeEnum::LocalFs,
            "aws_s3" | "s3" | "S3" => crate::sea_orm_active_enums::StorageTypeEnum::AwsS3,
            _ => crate::sea_orm_active_enums::StorageTypeEnum::Database,
        };
        Self {
            id: model.id,
            sha1: model.sha1,
            content: model.content,
            file_type: model.file_type,
            storage_type: match model.storage_type {
                sea_orm::ActiveValue::Set(v) => sea_orm::ActiveValue::Set(map_storage(v)),
                sea_orm::ActiveValue::Unchanged(v) => {
                    sea_orm::ActiveValue::Unchanged(map_storage(v))
                }
                sea_orm::ActiveValue::NotSet => sea_orm::ActiveValue::NotSet,
            },
            data: model.data,
            local_path: model.local_path,
            remote_url: model.remote_url,
            created_at: model.created_at,
        }
    }
}

// -------------------------
// mega_tag adapters
// -------------------------
impl From<mercury::internal::model::sea_models::mega_tag::Model> for crate::mega_tag::Model {
    fn from(model: mercury::internal::model::sea_models::mega_tag::Model) -> Self {
        Self {
            id: model.id,
            tag_id: model.tag_id,
            object_id: model.object_id,
            object_type: model.object_type,
            tag_name: model.tag_name,
            tagger: model.tagger,
            message: model.message,
            created_at: model.created_at,
        }
    }
}

impl From<mercury::internal::model::sea_models::mega_tag::ActiveModel>
    for crate::mega_tag::ActiveModel
{
    fn from(model: mercury::internal::model::sea_models::mega_tag::ActiveModel) -> Self {
        Self {
            id: model.id,
            tag_id: model.tag_id,
            object_id: model.object_id,
            object_type: model.object_type,
            tag_name: model.tag_name,
            tagger: model.tagger,
            message: model.message,
            created_at: model.created_at,
        }
    }
}

// -------------------------
// git_tree adapters
// -------------------------
impl From<mercury::internal::model::sea_models::git_tree::Model> for crate::git_tree::Model {
    fn from(model: mercury::internal::model::sea_models::git_tree::Model) -> Self {
        Self {
            id: model.id,
            repo_id: model.repo_id as i64,
            tree_id: model.tree_id,
            sub_trees: model.sub_trees,
            size: model.size,
            created_at: model.created_at,
        }
    }
}

impl From<mercury::internal::model::sea_models::git_tree::ActiveModel>
    for crate::git_tree::ActiveModel
{
    fn from(model: mercury::internal::model::sea_models::git_tree::ActiveModel) -> Self {
        Self {
            id: model.id,
            repo_id: match model.repo_id {
                sea_orm::ActiveValue::Set(v) => sea_orm::ActiveValue::Set(v as i64),
                sea_orm::ActiveValue::Unchanged(v) => sea_orm::ActiveValue::Unchanged(v as i64),
                sea_orm::ActiveValue::NotSet => sea_orm::ActiveValue::NotSet,
            },
            tree_id: model.tree_id,
            sub_trees: model.sub_trees,
            size: model.size,
            created_at: model.created_at,
        }
    }
}

// -------------------------
// git_blob adapters
// -------------------------
impl From<mercury::internal::model::sea_models::git_blob::Model> for crate::git_blob::Model {
    fn from(model: mercury::internal::model::sea_models::git_blob::Model) -> Self {
        Self {
            id: model.id,
            repo_id: model.repo_id as i64,
            blob_id: model.blob_id,
            name: model.name,
            size: model.size,
            created_at: model.created_at,
        }
    }
}

impl From<mercury::internal::model::sea_models::git_blob::ActiveModel>
    for crate::git_blob::ActiveModel
{
    fn from(model: mercury::internal::model::sea_models::git_blob::ActiveModel) -> Self {
        Self {
            id: model.id,
            repo_id: match model.repo_id {
                sea_orm::ActiveValue::Set(v) => sea_orm::ActiveValue::Set(v as i64),
                sea_orm::ActiveValue::Unchanged(v) => sea_orm::ActiveValue::Unchanged(v as i64),
                sea_orm::ActiveValue::NotSet => sea_orm::ActiveValue::NotSet,
            },
            blob_id: model.blob_id,
            name: model.name,
            size: model.size,
            created_at: model.created_at,
        }
    }
}

// -------------------------
// git_tag adapters
// -------------------------
impl From<mercury::internal::model::sea_models::git_tag::Model> for crate::git_tag::Model {
    fn from(model: mercury::internal::model::sea_models::git_tag::Model) -> Self {
        Self {
            id: model.id,
            repo_id: model.repo_id as i64,
            tag_id: model.tag_id,
            object_id: model.object_id,
            object_type: model.object_type,
            tag_name: model.tag_name,
            tagger: model.tagger,
            message: model.message,
            created_at: model.created_at,
        }
    }
}

impl From<mercury::internal::model::sea_models::git_tag::ActiveModel>
    for crate::git_tag::ActiveModel
{
    fn from(model: mercury::internal::model::sea_models::git_tag::ActiveModel) -> Self {
        Self {
            id: model.id,
            repo_id: match model.repo_id {
                sea_orm::ActiveValue::Set(v) => sea_orm::ActiveValue::Set(v as i64),
                sea_orm::ActiveValue::Unchanged(v) => sea_orm::ActiveValue::Unchanged(v as i64),
                sea_orm::ActiveValue::NotSet => sea_orm::ActiveValue::NotSet,
            },
            tag_id: model.tag_id,
            object_id: model.object_id,
            object_type: model.object_type,
            tag_name: model.tag_name,
            tagger: model.tagger,
            message: model.message,
            created_at: model.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(deprecated)]
    use chrono::NaiveDateTime;
    use sea_orm::ActiveValue;

    #[test]
    fn test_convert_git_commit_model() {
        // Create a mercury GitCommitModel
        let mercury_model = mercury::internal::model::sea_models::git_commit::Model {
            id: 1,
            repo_id: 123,
            commit_id: "commit123".to_string(),
            tree: "tree123".to_string(),
            parents_id: r#"["parent1", "parent2"]"#.to_string(),
            author: "author123".to_string().into(),
            committer: "committer123".to_string().into(),
            content: "content123".to_string().into(),
            created_at: NaiveDateTime::from_timestamp_opt(1609459200, 0).unwrap(), // 2021-01-01 00:00:00
        };

        // Convert to callisto model
        let callisto_model: crate::git_commit::Model = mercury_model.into();

        // Verify the conversion
        assert_eq!(callisto_model.id, 1);
        assert_eq!(callisto_model.repo_id, 123i64); // Should be converted from i32 to i64
        assert_eq!(callisto_model.commit_id, "commit123");
        assert_eq!(callisto_model.tree, "tree123");
        assert_eq!(callisto_model.author, "author123".to_string().into());
        assert_eq!(callisto_model.committer, "committer123".to_string().into());
        assert_eq!(callisto_model.content, "content123".to_string().into());
        assert_eq!(
            callisto_model.created_at,
            NaiveDateTime::from_timestamp_opt(1609459200, 0).unwrap()
        );

        // Verify parents_id conversion
        let parents: Vec<String> = serde_json::from_value(callisto_model.parents_id).unwrap();
        assert_eq!(parents, vec!["parent1", "parent2"]);
    }

    #[test]
    fn test_convert_git_commit_active_model() {
        // Create a mercury GitCommitActiveModel
        let mercury_active_model = mercury::internal::model::sea_models::git_commit::ActiveModel {
            id: ActiveValue::Set(1),
            repo_id: ActiveValue::Set(123),
            commit_id: ActiveValue::Set("commit123".to_string()),
            tree: ActiveValue::Set("tree123".to_string()),
            parents_id: ActiveValue::Set(r#"["parent1", "parent2"]"#.to_string()),
            author: ActiveValue::Set("author123".to_string().into()),
            committer: ActiveValue::Set("committer123".to_string().into()),
            content: ActiveValue::Set("content123".to_string().into()),
            created_at: ActiveValue::Set(NaiveDateTime::from_timestamp_opt(1609459200, 0).unwrap()),
        };

        // Convert to callisto active model
        let callisto_active_model: crate::git_commit::ActiveModel = mercury_active_model.into();

        // Verify the conversion
        match callisto_active_model.id {
            ActiveValue::Set(val) => assert_eq!(val, 1),
            _ => panic!("Expected ActiveValue::Set"),
        }

        match callisto_active_model.repo_id {
            ActiveValue::Set(val) => assert_eq!(val, 123i64), // Should be converted from i32 to i64
            _ => panic!("Expected ActiveValue::Set"),
        }

        match callisto_active_model.commit_id {
            ActiveValue::Set(ref val) => assert_eq!(val, "commit123"),
            _ => panic!("Expected ActiveValue::Set"),
        }

        match callisto_active_model.tree {
            ActiveValue::Set(ref val) => assert_eq!(val, "tree123"),
            _ => panic!("Expected ActiveValue::Set"),
        }

        match callisto_active_model.author {
            ActiveValue::Set(ref val) => assert_eq!(val, &Some("author123".to_string())),
            _ => panic!("Expected ActiveValue::Set"),
        }

        match callisto_active_model.committer {
            ActiveValue::Set(ref val) => assert_eq!(val, &Some("committer123".to_string())),
            _ => panic!("Expected ActiveValue::Set"),
        }

        match callisto_active_model.content {
            ActiveValue::Set(ref val) => assert_eq!(val, &Some("content123".to_string())),
            _ => panic!("Expected ActiveValue::Set"),
        }

        match callisto_active_model.created_at {
            ActiveValue::Set(val) => assert_eq!(
                val,
                NaiveDateTime::from_timestamp_opt(1609459200, 0).unwrap()
            ),
            _ => panic!("Expected ActiveValue::Set"),
        }

        // Verify parents_id conversion
        match callisto_active_model.parents_id {
            ActiveValue::Set(ref val) => {
                let val = val.as_str().unwrap();
                let parents: Vec<String> = serde_json::from_str(val).unwrap();
                assert_eq!(parents, vec!["parent1", "parent2"]);
            }
            _ => panic!("Expected ActiveValue::Set"),
        }
    }

    #[test]
    fn test_convert_mega_commit_model() {
        // Create a mercury MegaCommitModel
        let mercury_model = mercury::internal::model::sea_models::mega_commit::Model {
            id: 1,
            commit_id: "commit123".to_string(),
            tree: "tree123".to_string(),
            parents_id: r#"["parent1", "parent2"]"#.to_string(),
            author: "author123".to_string().into(),
            committer: "committer123".to_string().into(),
            content: "content123".to_string().into(),
            created_at: NaiveDateTime::from_timestamp_opt(1609459200, 0).unwrap(), // 2021-01-01 00:00:00
        };

        // Convert to callisto model
        let callisto_model: crate::mega_commit::Model = mercury_model.into();

        // Verify the conversion
        assert_eq!(callisto_model.id, 1);
        assert_eq!(callisto_model.commit_id, "commit123");
        assert_eq!(callisto_model.tree, "tree123");
        assert_eq!(callisto_model.author, "author123".to_string().into());
        assert_eq!(callisto_model.committer, "committer123".to_string().into());
        assert_eq!(callisto_model.content, "content123".to_string().into());
        assert_eq!(
            callisto_model.created_at,
            NaiveDateTime::from_timestamp_opt(1609459200, 0).unwrap()
        );

        // Verify parents_id conversion
        let parents: Vec<String> = serde_json::from_value(callisto_model.parents_id).unwrap();
        assert_eq!(parents, vec!["parent1", "parent2"]);
    }

    #[test]
    fn test_convert_mega_commit_active_model() {
        // Create a mercury MegaCommitActiveModel
        let mercury_active_model = mercury::internal::model::sea_models::mega_commit::ActiveModel {
            id: ActiveValue::Set(1),
            commit_id: ActiveValue::Set("commit123".to_string()),
            tree: ActiveValue::Set("tree123".to_string()),
            parents_id: ActiveValue::Set(r#"["parent1", "parent2"]"#.to_string()),
            author: ActiveValue::Set("author123".to_string().into()),
            committer: ActiveValue::Set("committer123".to_string().into()),
            content: ActiveValue::Set("content123".to_string().into()),
            created_at: ActiveValue::Set(NaiveDateTime::from_timestamp_opt(1609459200, 0).unwrap()),
        };

        // Convert to callisto active model
        let callisto_active_model: crate::mega_commit::ActiveModel = mercury_active_model.into();

        // Verify the conversion
        match callisto_active_model.id {
            ActiveValue::Set(val) => assert_eq!(val, 1),
            _ => panic!("Expected ActiveValue::Set"),
        }

        match callisto_active_model.commit_id {
            ActiveValue::Set(ref val) => assert_eq!(val, "commit123"),
            _ => panic!("Expected ActiveValue::Set"),
        }

        match callisto_active_model.tree {
            ActiveValue::Set(ref val) => assert_eq!(val, "tree123"),
            _ => panic!("Expected ActiveValue::Set"),
        }

        match callisto_active_model.author {
            ActiveValue::Set(ref val) => assert_eq!(val, &Some("author123".to_string())),
            _ => panic!("Expected ActiveValue::Set"),
        }

        match callisto_active_model.committer {
            ActiveValue::Set(ref val) => assert_eq!(val, &Some("committer123".to_string())),
            _ => panic!("Expected ActiveValue::Set"),
        }

        match callisto_active_model.content {
            ActiveValue::Set(ref val) => assert_eq!(val, &Some("content123".to_string())),
            _ => panic!("Expected ActiveValue::Set"),
        }

        match callisto_active_model.created_at {
            ActiveValue::Set(val) => assert_eq!(
                val,
                NaiveDateTime::from_timestamp_opt(1609459200, 0).unwrap()
            ),
            _ => panic!("Expected ActiveValue::Set"),
        }

        // Verify parents_id conversion
        match callisto_active_model.parents_id {
            ActiveValue::Set(ref val) => {
                let val = val.as_str().unwrap();
                let parents: Vec<String> = serde_json::from_str(val).unwrap();
                assert_eq!(parents, vec!["parent1", "parent2"]);
            }
            _ => panic!("Expected ActiveValue::Set"),
        }
    }

    #[test]
    fn test_convert_with_empty_parents() {
        // Create a mercury GitCommitModel with empty parents
        let mercury_model = mercury::internal::model::sea_models::git_commit::Model {
            id: 1,
            repo_id: 123,
            commit_id: "commit123".to_string(),
            tree: "tree123".to_string(),
            parents_id: "[]".to_string(), // Empty array
            author: "author123".to_string().into(),
            committer: "committer123".to_string().into(),
            content: "content123".to_string().into(),
            created_at: NaiveDateTime::from_timestamp_opt(1609459200, 0).unwrap(),
        };

        // Convert to callisto model
        let callisto_model: crate::git_commit::Model = mercury_model.into();

        // Verify parents_id conversion
        let parents: Vec<String> = serde_json::from_value(callisto_model.parents_id).unwrap();
        assert_eq!(parents, Vec::<String>::new());
    }

    #[test]
    fn test_convert_with_invalid_json() {
        // Create a mercury GitCommitModel with invalid JSON
        let mercury_model = mercury::internal::model::sea_models::git_commit::Model {
            id: 1,
            repo_id: 123,
            commit_id: "commit123".to_string(),
            tree: "tree123".to_string(),
            parents_id: "invalid json".to_string(), // Invalid JSON
            author: "author123".to_string().into(),
            committer: "committer123".to_string().into(),
            content: "content123".to_string().into(),
            created_at: NaiveDateTime::from_timestamp_opt(1609459200, 0).unwrap(),
        };

        // Convert to callisto model - should not panic
        let callisto_model: crate::git_commit::Model = mercury_model.into();

        // Should default to empty array
        let parents: Vec<String> = serde_json::from_value(callisto_model.parents_id).unwrap();
        assert_eq!(parents, Vec::<String>::new());
    }
}
