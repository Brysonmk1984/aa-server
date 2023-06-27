use sea_orm_migration::prelude::*;
use sea_orm_migration::DbErr;
use sea_orm_migration::{sea_orm::Statement, SchemaManager};

pub async fn raw_sql_migration(
    manager: &SchemaManager<'_>,
    statement: Statement,
) -> Result<(), DbErr> {
    manager
        .get_connection()
        .execute(statement)
        .await
        .map(|_| ())
}

#[derive(Iden)]
pub enum Armies {
    Table,
    Id,
    Name,
    Lore,
    Size,
    Damage,
    Health,
    Range,
    Armor,
    ShieldRating,
    Flying,
}
