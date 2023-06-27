use sea_orm_migration::prelude::*;

use crate::utils::Armies;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Armies::Table)
                    .modify_column(
                        ColumnDef::new(Alias::new("shield_rating"))
                            .decimal_len(5, 2)
                            .default(0.0)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Armies::Table)
                    .modify_column(
                        ColumnDef::new(Alias::new("shield_rating"))
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }
}
