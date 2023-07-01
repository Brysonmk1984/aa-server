use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::utils::raw_sql_migration;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            ALTER TABLE armies 
                ALTER COLUMN shield_rating type numeric(3,2),
                DROP COLUMN health,
                DROP COLUMN damage,
                DROP COLUMN armor,
                ADD COLUMN attack_speed numeric(3,2) NOT NULL DEFAULT 0.00,
                ADD COLUMN accuracy numeric(3,2) NOT NULL DEFAULT 0.00,
                ADD COLUMN aoe BOOLEAN NOT NULL DEFAULT FALSE,
                ADD COLUMN weapon_type char(20) NOT NULL DEFAULT 'piercing',
                ADD COLUMN armor_type char(20) NOT NULL DEFAULT 'unarmored',
                ADD COLUMN agility numeric(3,2) NOT NULL DEFAULT 0.00,
                ADD COLUMN speed int NOT NULL DEFAULT 1;

        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            ALTER TABLE armies 
                ALTER COLUMN shield_rating type numeric(5,2),
                ADD COLUMN health int NOT NULL DEFAULT 100,
                ADD COLUMN damage int NOT NULL DEFAULT 100,
                ADD COLUMN armor int NOT NULL DEFAULT 100,
                DROP COLUMN attack_speed,
                DROP COLUMN accuracy,
                DROP COLUMN aoe,
                DROP COLUMN weapon_type,
                DROP COLUMN armor_type,
                DROP COLUMN agility,
                DROP COLUMN speed;
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }
}
