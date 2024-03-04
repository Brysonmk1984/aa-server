use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::utils::raw_sql_migration;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            INSERT INTO weapon_armor
            VALUES 
               (DEFAULT, 'piercing', 'unarmored', 1.0),
                (DEFAULT, 'piercing', 'leather', 0.75),
                (DEFAULT, 'piercing', 'chain', 0.6),
                (DEFAULT, 'piercing', 'plate', 0.1),
                (DEFAULT, 'crushing', 'unarmored', 0.25),
                (DEFAULT, 'crushing', 'leather', 0.5),
                (DEFAULT, 'crushing', 'chain', 0.75),
                (DEFAULT, 'crushing', 'plate', 1.0),
                (DEFAULT, 'blunt', 'unarmored', 0.75),
                (DEFAULT, 'blunt', 'leather', 0.75),
                (DEFAULT, 'blunt', 'chain', 0.5),
                (DEFAULT, 'blunt', 'plate', 0.25),
                (DEFAULT, 'edged', 'unarmored', 1.0),
                (DEFAULT, 'edged', 'leather', 0.75),
                (DEFAULT, 'edged', 'chain', 0.5),
                (DEFAULT, 'edged', 'plate', 0.25),
                (DEFAULT, 'magic', 'unarmored', 0.25),
                (DEFAULT, 'magic', 'leather', 0.5),
                (DEFAULT, 'magic', 'chain', 1.0),
                (DEFAULT, 'magic', 'plate', 0.75)
            
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            DELETE FROM weapon_armor
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }
}
