use sea_orm_migration::{prelude::*, sea_orm::Statement};

use crate::utils::raw_sql_migration;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            INSERT INTO aoe_spread
            VALUES 
                (DEFAULT, 1, 0.0, 1),
                (DEFAULT, 1, 0.5, 2),
                (DEFAULT, 1, 1.0, 5),
                (DEFAULT, 1, 1.5, 9),
                (DEFAULT, 1, 2.0, 13),
                (DEFAULT, 1, 2.5, 20),
                (DEFAULT, 1, 3.0, 33),

                (DEFAULT, 2, 0.0, 1),
                (DEFAULT, 2, 0.5, 1),
                (DEFAULT, 2, 1.0, 2),
                (DEFAULT, 2, 1.5, 3),
                (DEFAULT, 2, 2.0, 5),
                (DEFAULT, 2, 2.5, 7),
                (DEFAULT, 2, 3.0, 9),


                (DEFAULT, 3, 0.0, 1),
                (DEFAULT, 3, 0.5, 1),
                (DEFAULT, 3, 1.0, 1),
                (DEFAULT, 3, 1.5, 2),
                (DEFAULT, 3, 2.0, 2),
                (DEFAULT, 3, 2.5, 3),
                (DEFAULT, 3, 3.0, 5)

        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "
            DELETE FROM aoe_spread
        ";
        let statement = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        raw_sql_migration(manager, statement).await
    }
}
