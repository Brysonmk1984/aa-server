use crate::utils::raw_sql_migration;
use sea_orm_migration::{prelude::*, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            CREATE TABLE nation_campaign_levels (
                id SERIAL PRIMARY KEY, 
                nation_id INT NOT NULL, 
                campaign_level_id INT NOT NULL, 
                nation_name VARCHAR(50),
                level INT NOT NULL,
                attempts INT NOT NULL,
               

                CONSTRAINT fk_nation_id
                    FOREIGN KEY(nation_id)
                        REFERENCES nations(id),

                CONSTRAINT fk_campaign_level_id
                    FOREIGN KEY(campaign_level_id)
                        REFERENCES campaign_levels(id),

                CONSTRAINT fk_nation_name
                    FOREIGN KEY(nation_name)
                        REFERENCES nations(name),

                CONSTRAINT fk_level
                    FOREIGN KEY(level)
                        REFERENCES campaign_levels(level)

            );
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            DROP TABLE nation_campaign_levels;
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }
}
