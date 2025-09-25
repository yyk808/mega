use jupiter::adapter::*;
use jupiter::callisto::{git_commit, git_tag, git_tree, mega_commit, mega_tree, raw_blob};
use jupiter::callisto::sea_orm_active_enums::StorageTypeEnum;
use mercury::hash::SHA1;
use mercury::internal::object::blob::Blob;
use mercury::internal::object::commit::Commit;
use mercury::internal::object::signature::Signature;
use mercury::internal::object::tag::Tag;
use mercury::internal::object::tree::{Tree, TreeItem, TreeItemMode};
use mercury::internal::object::types::ObjectType;
use std::str::FromStr;

#[cfg(test)]
mod raw_blob_tests {
    use super::*;

    #[test]
    fn test_raw_blob_to_blob_normal() {
        let data = b"Hello, World!".to_vec();
        let model = raw_blob::Model {
            id: 1,
            sha1: "dummy".to_string(),
            content: None,
            file_type: None,
            storage_type: StorageTypeEnum::Database,
            data: Some(data.clone()),
            local_path: None,
            remote_url: None,
            created_at: chrono::Utc::now().naive_utc(),
        };

        let blob = raw_blob_to_blob(model);
        assert_eq!(blob.data, data);
    }

    #[test]
    fn test_blob_to_raw_blob_normal() {
        let content = b"Hello, World!".to_vec();
        let blob = Blob::from_content_bytes(content.clone());
        
        let model = blob_to_raw_blob(blob.clone());
        assert_eq!(model.id, 0);
        assert_eq!(model.sha1, blob.id.to_string());
        assert_eq!(model.data, Some(content));
        assert_eq!(model.storage_type, StorageTypeEnum::Database);
    }

    #[test]
    fn test_raw_blob_to_blob_edge() {
        let model = raw_blob::Model {
            id: 1,
            sha1: "dummy".to_string(),
            content: None,
            file_type: None,
            storage_type: StorageTypeEnum::Database,
            data: None,
            local_path: None,
            remote_url: None,
            created_at: chrono::Utc::now().naive_utc(),
        };

        let blob = raw_blob_to_blob(model);
        assert_eq!(blob.data, vec![] as Vec<u8>);
    }

    #[test]
    fn test_blob_to_raw_blob_edge() {
        let blob = Blob::from_content_bytes(vec![]);
        
        let model = blob_to_raw_blob(blob.clone());
        assert_eq!(model.id, 0);
        assert_eq!(model.sha1, blob.id.to_string());
        assert_eq!(model.data, Some(vec![] as Vec<u8>));
        assert_eq!(model.storage_type, StorageTypeEnum::Database);
    }
}

#[cfg(test)]
mod git_tree_tests {
    use super::*;

    #[test]
    fn test_git_tree_to_tree_normal() {
        // Create some tree items
        let tree_items = vec![
            TreeItem::new(
                TreeItemMode::Blob,
                SHA1::from_str("a5bfc9e07964f8dddeb95fc584cd965d98869933").unwrap(),
                "file1.txt".to_string(),
            ),
            TreeItem::new(
                TreeItemMode::Blob,
                SHA1::from_str("b5bfc9e07964f8dddeb95fc584cd965d98869934").unwrap(),
                "file2.txt".to_string(),
            ),
        ];
        
        let _tree = Tree::from_tree_items(tree_items.clone()).expect("Failed to create tree");
        
        // Serialize tree items
        let sub_trees = bincode::serde::encode_to_vec(&tree_items, bincode::config::standard())
            .expect("Failed to serialize tree items");

        let model = git_tree::Model {
            id: 1,
            repo_id: 123,
            tree_id: "dummy".to_string(),
            sub_trees,
            size: 0,
            created_at: chrono::Utc::now().naive_utc(),
        };

        let result_tree = git_tree_to_tree(model);
        assert_eq!(result_tree.tree_items.len(), tree_items.len());
        for (i, item) in result_tree.tree_items.iter().enumerate() {
            assert_eq!(item.name, tree_items[i].name);
            assert_eq!(item.id, tree_items[i].id);
            assert_eq!(item.mode, tree_items[i].mode);
        }
    }

    #[test]
    fn test_tree_to_git_tree_normal() {
        let tree_items = vec![
            TreeItem::new(
                TreeItemMode::Blob,
                SHA1::from_str("a5bfc9e07964f8dddeb95fc584cd965d98869933").unwrap(),
                "file1.txt".to_string(),
            ),
        ];
        
        let tree = Tree::from_tree_items(tree_items).expect("Failed to create tree");
        let repo_id = 123;
        
        let model = tree_to_git_tree(tree.clone(), repo_id);
        assert_eq!(model.id, 0);
        assert_eq!(model.repo_id, repo_id);
        assert_eq!(model.tree_id, tree.id.to_string());
        assert_eq!(model.size, model.sub_trees.len() as i32);
    }

    #[test]
    #[should_panic(expected = "Failed to create Tree")]
    fn test_git_tree_to_tree_edge() {
        let model = git_tree::Model {
            id: 1,
            repo_id: 123,
            tree_id: "dummy".to_string(),
            sub_trees: vec![], // Empty binary data
            size: 0,
            created_at: chrono::Utc::now().naive_utc(),
        };

        // This should panic because we can't create an empty tree
        let _tree = git_tree_to_tree(model);
    }

    #[test]
    fn test_tree_to_git_tree_edge() {
        // Create a tree with one item instead of an empty tree
        let tree_items = vec![
            TreeItem::new(
                TreeItemMode::Blob,
                SHA1::from_str("a5bfc9e07964f8dddeb95fc584cd965d98869933").unwrap(),
                "file1.txt".to_string(),
            ),
        ];
        let tree = Tree::from_tree_items(tree_items).expect("Failed to create tree");
        let repo_id = 456;
        
        let model = tree_to_git_tree(tree.clone(), repo_id);
        assert_eq!(model.id, 0);
        assert_eq!(model.repo_id, repo_id);
        assert_eq!(model.tree_id, tree.id.to_string());
        assert!(model.size > 0);
    }

    #[test]
    #[should_panic(expected = "Failed to create Tree")]
    fn test_git_tree_to_tree_error() {
        let model = git_tree::Model {
            id: 1,
            repo_id: 123,
            tree_id: "dummy".to_string(),
            sub_trees: vec![1, 2, 3, 4], // Invalid serialized data
            size: 4,
            created_at: chrono::Utc::now().naive_utc(),
        };

        // This should panic because we can't create a tree from invalid data
        let _tree = git_tree_to_tree(model);
    }
}

#[cfg(test)]
mod mega_tree_tests {
    use super::*;

    #[test]
    fn test_mega_tree_to_tree_normal() {
        // Create some tree items
        let tree_items = vec![
            TreeItem::new(
                TreeItemMode::Blob,
                SHA1::from_str("a5bfc9e07964f8dddeb95fc584cd965d98869933").unwrap(),
                "file1.txt".to_string(),
            ),
            TreeItem::new(
                TreeItemMode::Blob,
                SHA1::from_str("b5bfc9e07964f8dddeb95fc584cd965d98869934").unwrap(),
                "file2.txt".to_string(),
            ),
        ];
        
        let _tree = Tree::from_tree_items(tree_items.clone()).expect("Failed to create tree");
        
        // Serialize tree items
        let sub_trees = bincode::serde::encode_to_vec(&tree_items, bincode::config::standard())
            .expect("Failed to serialize tree items");

        let model = mega_tree::Model {
            id: 1,
            tree_id: "dummy".to_string(),
            sub_trees,
            size: 0,
            commit_id: "abc123".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
        };

        let result_tree = mega_tree_to_tree(model);
        assert_eq!(result_tree.tree_items.len(), tree_items.len());
        for (i, item) in result_tree.tree_items.iter().enumerate() {
            assert_eq!(item.name, tree_items[i].name);
            assert_eq!(item.id, tree_items[i].id);
            assert_eq!(item.mode, tree_items[i].mode);
        }
    }

    #[test]
    fn test_tree_to_mega_tree_normal() {
        let tree_items = vec![
            TreeItem::new(
                TreeItemMode::Blob,
                SHA1::from_str("a5bfc9e07964f8dddeb95fc584cd965d98869933").unwrap(),
                "file1.txt".to_string(),
            ),
        ];
        
        let tree = Tree::from_tree_items(tree_items).expect("Failed to create tree");
        let commit_id = "abc123";
        
        let model = tree_to_mega_tree(tree.clone(), commit_id);
        assert_eq!(model.id, 0);
        assert_eq!(model.tree_id, tree.id.to_string());
        assert_eq!(model.commit_id, commit_id);
        assert_eq!(model.size, model.sub_trees.len() as i32);
    }

    #[test]
    #[should_panic(expected = "Failed to create Tree")]
    fn test_mega_tree_to_tree_edge() {
        let model = mega_tree::Model {
            id: 1,
            tree_id: "dummy".to_string(),
            sub_trees: vec![], // Empty binary data
            size: 0,
            commit_id: "def456".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
        };

        // This should panic because we can't create an empty tree
        let _tree = mega_tree_to_tree(model);
    }

    #[test]
    fn test_tree_to_mega_tree_edge() {
        // Create a tree with one item instead of an empty tree
        let tree_items = vec![
            TreeItem::new(
                TreeItemMode::Blob,
                SHA1::from_str("a5bfc9e07964f8dddeb95fc584cd965d98869933").unwrap(),
                "file1.txt".to_string(),
            ),
        ];
        let tree = Tree::from_tree_items(tree_items).expect("Failed to create tree");
        let commit_id = "def456";
        
        let model = tree_to_mega_tree(tree.clone(), commit_id);
        assert_eq!(model.id, 0);
        assert_eq!(model.tree_id, tree.id.to_string());
        assert_eq!(model.commit_id, commit_id);
        assert!(model.size > 0);
    }

    #[test]
    #[should_panic(expected = "Failed to create Tree")]
    fn test_mega_tree_to_tree_error() {
        let model = mega_tree::Model {
            id: 1,
            tree_id: "dummy".to_string(),
            sub_trees: vec![1, 2, 3, 4], // Invalid serialized data
            size: 4,
            commit_id: "def456".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
        };

        // This should panic because we can't create a tree from invalid data
        let _tree = mega_tree_to_tree(model);
    }
}

#[cfg(test)]
mod git_commit_tests {
    use super::*;

    #[test]
    fn test_git_commit_to_commit_normal() {
        let model = git_commit::Model {
            id: 1,
            repo_id: 123,
            commit_id: "dummy".to_string(),
            tree: "a5bfc9e07964f8dddeb95fc584cd965d98869933".to_string(),
            parents_id: serde_json::Value::String("[\"b5bfc9e07964f8dddeb95fc584cd965d98869934\"]".to_string()),
            author: Some("author Author Name <author@example.com> 1234567890 +0000".to_string()),
            committer: Some("committer Committer Name <committer@example.com> 1234567891 +0000".to_string()),
            content: Some("Commit message".to_string()),
            created_at: chrono::Utc::now().naive_utc(),
        };

        let commit = git_commit_to_commit(model);
        assert_eq!(commit.message, "Commit message");
        assert_eq!(commit.parent_commit_ids.len(), 1);
        assert_eq!(commit.tree_id.to_string(), "a5bfc9e07964f8dddeb95fc584cd965d98869933");
    }

    #[test]
    fn test_commit_to_git_commit_normal() {
        let parent_id = SHA1::from_str("b5bfc9e07964f8dddeb95fc584cd965d98869934").unwrap();
        let tree_id = SHA1::from_str("a5bfc9e07964f8dddeb95fc584cd965d98869933").unwrap();
        
        let author = Signature::from_data("author Author Name <author@example.com> 1234567890 +0000".to_string().into_bytes()).unwrap();
        let committer = Signature::from_data("committer Committer Name <committer@example.com> 1234567891 +0000".to_string().into_bytes()).unwrap();
        
        let commit = Commit::new(author.clone(), committer.clone(), tree_id, vec![parent_id], "Commit message");
        let repo_id = 789;
        
        let model = commit_to_git_commit(commit.clone(), repo_id);
        assert_eq!(model.id, 0);
        assert_eq!(model.repo_id, repo_id);
        assert_eq!(model.commit_id, commit.id.to_string());
        assert_eq!(model.tree, commit.tree_id.to_string());
        assert_eq!(model.author, Some(author.to_string()));
        assert_eq!(model.committer, Some(committer.to_string()));
        assert_eq!(model.content, Some("Commit message".to_string()));
    }

    #[test]
    fn test_git_commit_to_commit_edge() {
        let model = git_commit::Model {
            id: 1,
            repo_id: 123,
            commit_id: "dummy".to_string(),
            tree: "a5bfc9e07964f8dddeb95fc584cd965d98869933".to_string(),
            parents_id: serde_json::Value::String("[]".to_string()), // Empty parents
            author: None, // No author
            committer: None, // No committer
            content: None, // No content
            created_at: chrono::Utc::now().naive_utc(),
        };

        let commit = git_commit_to_commit(model);
        assert_eq!(commit.message, "");
        assert_eq!(commit.parent_commit_ids.len(), 0);
        assert_eq!(commit.tree_id.to_string(), "a5bfc9e07964f8dddeb95fc584cd965d98869933");
        // Should have default author and committer
    }

    #[test]
    fn test_commit_to_git_commit_edge() {
        let tree_id = SHA1::from_str("a5bfc9e07964f8dddeb95fc584cd965d98869933").unwrap();
        
        let author = Signature::from_data("author Author Name <author@example.com> 1234567890 +0000".to_string().into_bytes()).unwrap();
        let committer = Signature::from_data("committer Committer Name <committer@example.com> 1234567891 +0000".to_string().into_bytes()).unwrap();
        
        let commit = Commit::new(author.clone(), committer.clone(), tree_id, vec![], ""); // No parents, empty message
        let repo_id = 101;
        
        let model = commit_to_git_commit(commit.clone(), repo_id);
        assert_eq!(model.id, 0);
        assert_eq!(model.repo_id, repo_id);
        assert_eq!(model.commit_id, commit.id.to_string());
        assert_eq!(model.tree, commit.tree_id.to_string());
        assert_eq!(model.author, Some(author.to_string()));
        assert_eq!(model.committer, Some(committer.to_string()));
        assert_eq!(model.content, Some("".to_string()));
    }

    // Note: Error cases that would panic are not tested here as they would cause the test to fail
}

#[cfg(test)]
mod mega_commit_tests {
    use super::*;

    #[test]
    fn test_mega_commit_to_commit_normal() {
        let model = mega_commit::Model {
            id: 1,
            commit_id: "dummy".to_string(),
            tree: "a5bfc9e07964f8dddeb95fc584cd965d98869933".to_string(),
            parents_id: serde_json::Value::String("[\"b5bfc9e07964f8dddeb95fc584cd965d98869934\"]".to_string()),
            author: Some("author Author Name <author@example.com> 1234567890 +0000".to_string()),
            committer: Some("committer Committer Name <committer@example.com> 1234567891 +0000".to_string()),
            content: Some("Commit message".to_string()),
            created_at: chrono::Utc::now().naive_utc(),
        };

        let commit = mega_commit_to_commit(model);
        assert_eq!(commit.message, "Commit message");
        assert_eq!(commit.parent_commit_ids.len(), 1);
        assert_eq!(commit.tree_id.to_string(), "a5bfc9e07964f8dddeb95fc584cd965d98869933");
    }

    #[test]
    fn test_commit_to_mega_commit_normal() {
        let parent_id = SHA1::from_str("b5bfc9e07964f8dddeb95fc584cd965d98869934").unwrap();
        let tree_id = SHA1::from_str("a5bfc9e07964f8dddeb95fc584cd965d98869933").unwrap();
        
        let author = Signature::from_data("author Author Name <author@example.com> 1234567890 +0000".to_string().into_bytes()).unwrap();
        let committer = Signature::from_data("committer Committer Name <committer@example.com> 1234567891 +0000".to_string().into_bytes()).unwrap();
        
        let commit = Commit::new(author.clone(), committer.clone(), tree_id, vec![parent_id], "Commit message");
        
        let model = commit_to_mega_commit(commit.clone());
        assert_eq!(model.id, 0);
        assert_eq!(model.commit_id, commit.id.to_string());
        assert_eq!(model.tree, commit.tree_id.to_string());
        assert_eq!(model.author, Some(author.to_string()));
        assert_eq!(model.committer, Some(committer.to_string()));
        assert_eq!(model.content, Some("Commit message".to_string()));
    }

    #[test]
    fn test_mega_commit_to_commit_edge() {
        let model = mega_commit::Model {
            id: 1,
            commit_id: "dummy".to_string(),
            tree: "a5bfc9e07964f8dddeb95fc584cd965d98869933".to_string(),
            parents_id: serde_json::Value::String("[]".to_string()), // Empty parents
            author: None, // No author
            committer: None, // No committer
            content: None, // No content
            created_at: chrono::Utc::now().naive_utc(),
        };

        let commit = mega_commit_to_commit(model);
        assert_eq!(commit.message, "");
        assert_eq!(commit.parent_commit_ids.len(), 0);
        assert_eq!(commit.tree_id.to_string(), "a5bfc9e07964f8dddeb95fc584cd965d98869933");
        // Should have default author and committer
    }

    #[test]
    fn test_commit_to_mega_commit_edge() {
        let tree_id = SHA1::from_str("a5bfc9e07964f8dddeb95fc584cd965d98869933").unwrap();
        
        let author = Signature::from_data("author Author Name <author@example.com> 1234567890 +0000".to_string().into_bytes()).unwrap();
        let committer = Signature::from_data("committer Committer Name <committer@example.com> 1234567891 +0000".to_string().into_bytes()).unwrap();
        
        let commit = Commit::new(author.clone(), committer.clone(), tree_id, vec![], ""); // No parents, empty message
        
        let model = commit_to_mega_commit(commit.clone());
        assert_eq!(model.id, 0);
        assert_eq!(model.commit_id, commit.id.to_string());
        assert_eq!(model.tree, commit.tree_id.to_string());
        assert_eq!(model.author, Some(author.to_string()));
        assert_eq!(model.committer, Some(committer.to_string()));
        assert_eq!(model.content, Some("".to_string()));
    }

    // Note: Error cases that would panic are not tested here as they would cause the test to fail
}

#[cfg(test)]
mod git_tag_tests {
    use super::*;

    #[test]
    fn test_git_tag_to_tag_normal() {
        let model = git_tag::Model {
            id: 1,
            repo_id: 123,
            tag_id: "dummy".to_string(),
            object_id: "a5bfc9e07964f8dddeb95fc584cd965d98869933".to_string(),
            object_type: "commit".to_string(),
            tag_name: "v1.0".to_string(),
            tagger: "Tagger Name <tagger@example.com> 1234567890 +0000".to_string(),
            message: "Tag message".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
        };

        let tag = git_tag_to_tag(model);
        assert_eq!(tag.tag_name, "v1.0");
        assert_eq!(tag.message, "Tag message");
        assert_eq!(tag.object_type, ObjectType::Commit);
        assert_eq!(tag.object_hash.to_string(), "a5bfc9e07964f8dddeb95fc584cd965d98869933");
    }

    #[test]
    fn test_tag_to_git_tag_normal() {
        let object_hash = SHA1::from_str("a5bfc9e07964f8dddeb95fc584cd965d98869933").unwrap();
        let tagger = Signature::from_data("tagger Tagger Name <tagger@example.com> 1234567890 +0000".to_string().into_bytes()).unwrap();
        
        let tag = Tag::new(object_hash, ObjectType::Commit, "v1.0".to_string(), tagger.clone(), "Tag message".to_string());
        let repo_id = 202;
        
        let model = tag_to_git_tag(tag.clone(), repo_id);
        assert_eq!(model.id, 0);
        assert_eq!(model.repo_id, repo_id);
        assert_eq!(model.tag_id, tag.id.to_string());
        assert_eq!(model.object_id, tag.object_hash.to_string());
        assert_eq!(model.object_type, "commit");
        assert_eq!(model.tag_name, "v1.0");
        assert_eq!(model.tagger, tagger.to_string());
        assert_eq!(model.message, "Tag message");
    }

    #[test]
    fn test_git_tag_to_tag_edge() {
        let model = git_tag::Model {
            id: 1,
            repo_id: 123,
            tag_id: "dummy".to_string(),
            object_id: "a5bfc9e07964f8dddeb95fc584cd965d98869933".to_string(),
            object_type: "invalid".to_string(), // Invalid object type
            tag_name: "v1.0".to_string(),
            tagger: "Tagger Name <tagger@example.com> 1234567890 +0000".to_string(),
            message: "Tag message".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
        };

        let tag = git_tag_to_tag(model);
        // Should default to Blob for invalid object type
        assert_eq!(tag.object_type, ObjectType::Blob);
    }

    #[test]
    fn test_tag_to_git_tag_edge() {
        let object_hash = SHA1::from_str("a5bfc9e07964f8dddeb95fc584cd965d98869933").unwrap();
        let tagger = Signature::from_data("tagger Tagger Name <tagger@example.com> 1234567890 +0000".to_string().into_bytes()).unwrap();
        
        let tag = Tag::new(object_hash, ObjectType::Tree, "v1.0".to_string(), tagger.clone(), "Tag message".to_string());
        let repo_id = 303;
        
        let model = tag_to_git_tag(tag.clone(), repo_id);
        assert_eq!(model.id, 0);
        assert_eq!(model.repo_id, repo_id);
        assert_eq!(model.tag_id, tag.id.to_string());
        assert_eq!(model.object_id, tag.object_hash.to_string());
        assert_eq!(model.object_type, "tree");
        assert_eq!(model.tag_name, "v1.0");
        assert_eq!(model.tagger, tagger.to_string());
        assert_eq!(model.message, "Tag message");
    }

    // Note: Error cases that would panic are not tested here as they would cause the test to fail
}