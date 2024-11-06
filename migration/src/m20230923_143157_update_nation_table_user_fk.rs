use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::utils::raw_sql_migration;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            ALTER TABLE nations
            ADD CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            ALTER TABLE nations
            DROP CONSTRAINT fk_user
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }
}
