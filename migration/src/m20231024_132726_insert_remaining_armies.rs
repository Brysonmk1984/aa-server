use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::utils::raw_sql_migration;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            INSERT INTO armies VALUES 
                (DEFAULT, 'Barbarians of the Outer Steppe', 'TBD', 100, 0.00, FALSE, 5, 0.90, 'crushing', 'leather', 0.33, 10, 0.00, 2.00, 1),
                (DEFAULT, 'Oath-Sworn Knights', 'TBD', 100, 0.40, FALSE, 5, 0.90,  'edged', 'plate', 0.15, 5, 0.00, 2.00, 1),
                (DEFAULT, 'Minute Men Militia', 'TBD', 100, 0.00, FALSE, 5, 0.75, 'edged', 'unarmored', 0.25, 10, 0.00, 1.00, 1),
                (DEFAULT, 'Death Dealer Assassins', 'TBD', 100, 0.00, FALSE, 10, 1.00, 'edged', 'unarmored', 0.35, 12, 0.00, 3.00, 2),
                (DEFAULT, 'Elven Archers', 'TBD', 100, 0.00, FALSE, 75, 0.95,  'piercing', 'leather', 0.25, 12, 0.00, 2.00, 1),
                (DEFAULT, 'Castlegate Crossbowmen', 'TBD', 100, 0.00, FALSE, 50, 0.70, 'piercing', 'chain', 0.15, 10, 0.00, 1.00, 2)
            ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            DELETE FROM armies 
                WHERE name='Barbarians of the Outer Steppe'
                OR name='Oath-Sworn Knights'
                OR name='Minute Men Militia'
                OR name='Death Dealer Assassins'
                OR name='Elven Archers' 
                OR name='Castlegate Crossbowmen'
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }
}
