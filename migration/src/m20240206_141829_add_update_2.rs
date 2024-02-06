use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::utils::raw_sql_migration;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            CREATE TRIGGER update_updated_at
            BEFORE UPDATE
            ON
                users
            FOR EACH ROW
                EXECUTE PROCEDURE update_updated_at_task();
            ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            DROP TRIGGER update_updated_at ON users;
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }
}
