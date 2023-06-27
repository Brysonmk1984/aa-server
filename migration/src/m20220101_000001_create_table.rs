use sea_orm_migration::prelude::*;

use crate::utils::Armies;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Armies::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Armies::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Armies::Name).string().not_null())
                    .col(ColumnDef::new(Armies::Lore).string().not_null())
                    .col(ColumnDef::new(Armies::Size).integer().not_null())
                    .col(ColumnDef::new(Armies::Damage).integer().not_null())
                    .col(ColumnDef::new(Armies::Health).integer().not_null())
                    .col(ColumnDef::new(Armies::Armor).integer().not_null())
                    .col(ColumnDef::new(Armies::ShieldRating).integer().not_null())
                    .col(ColumnDef::new(Armies::Flying).boolean().not_null())
                    .col(ColumnDef::new(Armies::Range).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Armies::Table).to_owned())
            .await
    }
}
