use crate::utils::raw_sql_migration;
use sea_orm_migration::{prelude::*, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            CREATE TABLE nations (id SERIAL PRIMARY KEY, user_id INT, name VARCHAR(50) NOT NULL, gold INT NOT NULL DEFAULT 0, is_npc BOOLEAN NOT NULL DEFAULT FALSE, lore VARCHAR(150));
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            DROP TABLE nations;
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }
}
