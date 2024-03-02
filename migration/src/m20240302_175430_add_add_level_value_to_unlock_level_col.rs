use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::utils::raw_sql_migration;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            UPDATE ARMIES as a set
            name = c.name,
            unlock_level = c.unlock_level
            from (values
                ('Peacekeeper Monks', 2),
                ('North Watch Longbowmen', 3),
                ('Barbarians of the Outer Steppe', 4),
                ('Amazonian Huntresses', 6),
                ('Rōnin Immortals', 8),
                ('Castlegate Crossbowmen', 10),
                ('Oath-Sworn Knights', 12),
                ('Shinobi Martial Artists', 14),
                ('Avian Cliff Dwellers', 16),
                ('Highborn Cavalry', 18),
                ('Death Dealer Assassins', 20),
                ('Magi Enforcers', 22),
                ('Elven Archers', 24),
                ('Imperial Legionnaires', 26),
                ('Skull Clan Death Cultists', 28)
            ) as c(name, unlock_level)
            WHERE c.name = a.name;
            ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            UPDATE ARMIES
            SET unlock_level = 0
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }
}
