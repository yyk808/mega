//! Database migration module for the Jupiter application.
//!
//! This module provides database migration functionality using SeaORM's migration framework.
//! It contains all migration files and utilities for managing database schema changes.
//!
//! # Overview
//!
//! The migrator handles database schema evolution through versioned migration files.
//! Each migration is represented as a separate module and implements the `MigrationTrait`.
//!
//! # Migration Files
//!
//! - `m20250314_025943_init` - Initial database schema setup
//! - `m20250427_031332_add_mr_refs_tag` - Adds merge request reference tagging
//! - `m20250605_013340_alter_mega_mr_index` - Modifies merge request indexing
//! - `m20250610_000001_add_vault_storage` - Adds vault storage functionality
//!
//! # Usage
//!
//! ```rust
//! use jupiter::migrator::apply_migrations;
//!
//! // Apply pending migrations
//! apply_migrations(&db, false).await?;
//!
//! // Refresh all migrations (development only)
//! apply_migrations(&db, true).await?;
//! ```
//!
//! # Safety
//!

/// Creates a primary key column definition with big integer type.
///
/// # Arguments
///
/// * `name` - The name of the column that implements `IntoIden`
///
/// # Returns
///
/// A `ColumnDef` configured as a primary key big integer column

/// The main migrator struct that implements the migration trait.
///
/// This struct is responsible for managing all database migrations in the correct order.

/// Applies database migrations to the given database connection.
///
/// # Arguments
///
/// * `db` - Reference to the database connection
/// * `refresh` - If true, refreshes all migrations (drops and recreates). If false, applies pending migrations only
///
/// # Returns
///
/// * `Ok(())` - If migrations were applied successfully
/// * `Err(MegaError)` - If migration failed, with error details logged
///
/// # Errors
///
/// Returns `MegaError` when:
/// - Database connection fails
/// - Migration SQL execution fails
/// - Schema validation errors occur
use sea_orm::DatabaseConnection;
use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::big_integer;
use tracing::log;

use common::errors::MegaError;

mod m20250314_025943_init;
mod m20250427_031332_add_mr_refs_tag;
mod m20250605_013340_alter_mega_mr_index;
mod m20250610_000001_add_vault_storage;

pub(self) fn pk_bigint<T: IntoIden>(name: T) -> ColumnDef {
    big_integer(name).primary_key().take()
}

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250314_025943_init::Migration),
            Box::new(m20250427_031332_add_mr_refs_tag::Migration),
            Box::new(m20250605_013340_alter_mega_mr_index::Migration),
            Box::new(m20250610_000001_add_vault_storage::Migration),
        ]
    }
}


pub async fn apply_migrations(
    db: &DatabaseConnection,
    refresh: bool,
) -> Result<(), MegaError> {
    match refresh {
        true => Migrator::refresh(db).await,
        false => Migrator::up(db, None).await,
    }
    .map_err(|e| {
        log::error!("Failed to apply migrations: {}", e);
        e.into()
    })
}
