use crate::utils::raw_sql_migration;
use sea_orm_migration::{prelude::*, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            CREATE TABLE battles (
                id SERIAL PRIMARY KEY, 
                nation_id_east INT NOT NULL, 
                nation_id_west INT NOT NULL, 
                CONSTRAINT fk_east_id
                    FOREIGN KEY(nation_id_east)
                        REFERENCES nations(id),

                CONSTRAINT fk_west_id
                    FOREIGN KEY(nation_id_west)
                        REFERENCES nations(id)
            );
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            DROP TABLE battles;
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }
}
