use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::utils::raw_sql_migration;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            ALTER TABLE battles
                ADD CONSTRAINT fk_nation_campaign_level_id
                        FOREIGN KEY(nation_campaign_level_id)
                            REFERENCES nation_campaign_levels(id) ON DELETE CASCADE;
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            ALTER TABLE battles
                DROP CONSTRAINT fk_nation_campaign_level_id;
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }
}
