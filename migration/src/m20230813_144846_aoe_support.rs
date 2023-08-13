use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::utils::raw_sql_migration;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            ALTER TABLE armies
                DROP COLUMN aoe,
                ADD COLUMN aoe NUMERIC(4,2) DEFAULT '00.00',
                ADD COLUMN spread NUMERIC DEFAULT '01.00';
        ";

        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            ALTER TABLE armies
                DROP COLUMN aoe,
                DROP COLUMN spread,
                ADD COLUMN aoe BOOLEAN DEFAULT false;
        ";

        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }
}
