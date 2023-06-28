use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::utils::raw_sql_migration;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "INSERT INTO armies VALUES (DEFAULT, 'North Glade Longbowmen', 'TBD', 500, 100, 35, 0, 0.0, false, 100)";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }

    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "INSERT INTO armies VALUES (DEFAULT, 'Highborn Calvary', 'TBD', 250, 75, 100, 50, 5.5, false, 10)";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }
}
