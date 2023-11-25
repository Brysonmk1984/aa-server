//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "battles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub nation_id_east: i32,
    pub nation_id_west: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::nations::Entity",
        from = "Column::NationIdEast",
        to = "super::nations::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Nations2,
    #[sea_orm(
        belongs_to = "super::nations::Entity",
        from = "Column::NationIdWest",
        to = "super::nations::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Nations1,
}

impl ActiveModelBehavior for ActiveModel {}
