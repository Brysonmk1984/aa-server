use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::utils::raw_sql_migration;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            INSERT INTO armies VALUES 
                (DEFAULT, 'Peacekeeper Monks', 'TBD', 1, 0.00, FALSE, 1, 0.00, 0.00, FALSE, 'piercing', 'unarmored', 0.00, 1),
                (DEFAULT, 'Imperial Legionnaires', 'TBD', 1, 0.00, FALSE, 1, 0.00, 0.00, FALSE, 'piercing', 'unarmored', 0.00, 1),
                (DEFAULT, 'North Watch Longbowmen', 'TBD', 1, 0.00, FALSE, 1, 0.00, 0.00, FALSE, 'piercing', 'unarmored', 0.00, 1),
                (DEFAULT, 'Highborn Cavalry', 'TBD', 1, 0.00, FALSE, 1, 0.00, 0.00, FALSE, 'piercing', 'unarmored', 0.00, 1),

                (DEFAULT, 'Rōnin Immortals', 'TBD', 1, 0.00, FALSE, 1, 0.00, 0.00, FALSE, 'piercing', 'unarmored', 0.00, 1),
                (DEFAULT, 'Shinobi Martial Artists', 'TBD', 1, 0.00, FALSE, 1, 0.00, 0.00, FALSE, 'piercing', 'unarmored', 0.00, 1),

                (DEFAULT, 'Amazonian Huntresses', 'TBD', 1, 0.00, FALSE, 1, 0.00, 0.00, FALSE, 'piercing', 'unarmored', 0.00, 1),
                (DEFAULT, 'Avian Cliff Dwellers', 'TBD', 1, 0.00, FALSE, 1, 0.00, 0.00, FALSE, 'piercing', 'unarmored', 0.00, 1),
                (DEFAULT, 'Magi Enforcers', 'TBD', 1, 0.00, FALSE, 1, 0.00, 0.00, FALSE, 'piercing', 'unarmored', 0.00, 1),
                (DEFAULT, 'Skull Clan Death Cultists', 'TBD', 1, 0.00, FALSE, 1, 0.00, 0.00, FALSE, 'piercing', 'unarmored', 0.00, 1)
                
            ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            DELETE FROM armies 
                WHERE name='Peacekeeper Monks'
                OR name='Imperial Legionnaires'
                OR name='North Watch Longbowmen'
                OR name='Highborn Cavalry'
                OR name='Rōnin Immortals' 
                OR name='Shinobi Assassins'
                OR name='Amazonian Huntresses'
                OR name='Avian Cliff Dwellers'
                OR name='Magi Enforcers'
                OR name='Skull Clan Death Cultists'
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }
}
