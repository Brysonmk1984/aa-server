use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::utils::raw_sql_migration;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "INSERT INTO armies VALUES (DEFAULT, 'North Glade Longbowmen', 'TBD', 500, 100,false,  35, 0, 0.0, 100), (DEFAULT, 'Highborn Calvary', 'TBD', 250, 75, false,  100, 50, 5.5,10)";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql =
            "DELETE FROM armies WHERE name IN ('North Glade Longbowmen', 'Imperial Legionnaire')";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }
}
