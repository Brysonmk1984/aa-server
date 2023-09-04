use crate::utils::raw_sql_migration;
use sea_orm_migration::{prelude::*, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            CREATE TABLE nation_armies (
                id SERIAL PRIMARY KEY,
                nation_id INT NOT NULL,
                army_id INT NOT NULL,
                count INT NOT NULL DEFAULT 0,
                army_name VARCHAR (50) NOT NULL,
                CONSTRAINT fk_nation FOREIGN KEY(nation_id) REFERENCES nations(id),
                CONSTRAINT fk_army FOREIGN KEY(army_id) REFERENCES armies(id),
                CONSTRAINT fk_name FOREIGN KEY(army_name) REFERENCES armies(name)
            );
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            DROP TABLE nation_armies;
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }
}
