use crate::utils::raw_sql_migration;
use sea_orm_migration::{prelude::*, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            CREATE TABLE campaign_levels (
                id SERIAL PRIMARY KEY, 
                nation_id INT NOT NULL, 
                nation_name VARCHAR(50) NOT NULL, 
                level INT UNIQUE,
      
               
                CONSTRAINT fk_nation_id
                    FOREIGN KEY(nation_id)
                        REFERENCES nations(id) ON DELETE CASCADE,

                CONSTRAINT fk_nation_name
                    FOREIGN KEY(nation_name)
                        REFERENCES nations(name) ON DELETE CASCADE

            );
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            DROP TABLE campaign_levels;
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }
}
