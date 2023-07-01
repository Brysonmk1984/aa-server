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
    ShieldRating,
    Flying,
    Range,
    Accuracy,
    Aoe,
    WeaponType,
    ArmorType,
    Agility,
    Speed,
}
// pub id: i32,
// pub name: String,
// pub lore: String,
// pub size: i32,
// #[sea_orm(column_type = "Decimal(Some((3, 2)))")]
// pub shield_rating: Decimal,
// pub flying: bool,
// pub range: i32,

// #[sea_orm(column_type = "Decimal(Some((3, 2)))")]
// pub attack_speed: Decimal,
// #[sea_orm(column_type = "Decimal(Some((3, 2)))")]
// pub accuracy: Decimal,
// pub aoe: bool,

// pub weapon_type: String,
// pub armor_type: String,
// #[sea_orm(column_type = "Decimal(Some((3, 2)))")]
// pub agility: Decimal,
// pub speed: i32,
