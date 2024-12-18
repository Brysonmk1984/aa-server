//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "armies")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub name: String,
    pub lore: String,
    pub count: i32,
    #[sea_orm(column_type = "Decimal(Some((3, 2)))")]
    pub shield_rating: Decimal,
    pub flying: bool,
    pub range: i32,
    #[sea_orm(column_type = "Decimal(Some((3, 2)))")]
    pub accuracy: Decimal,
    pub weapon_type: String,
    pub armor_type: String,
    #[sea_orm(column_type = "Decimal(Some((3, 2)))")]
    pub agility: Decimal,
    pub speed: i32,
    #[sea_orm(column_type = "Decimal(Some((4, 2)))", nullable)]
    pub aoe: Option<Decimal>,
    pub spread: Option<Decimal>,
    pub attack_speed: i32,
    pub cost: i32,
    pub unlock_level: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
