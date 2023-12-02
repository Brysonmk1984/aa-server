//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "campaign_levels")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub nation_id: i32,
    pub nation_name: String,
    #[sea_orm(unique)]
    pub level: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::nations::Entity",
        from = "Column::NationId",
        to = "super::nations::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Nations2,
    #[sea_orm(
        belongs_to = "super::nations::Entity",
        from = "Column::NationName",
        to = "super::nations::Column::Name",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Nations1,
}

impl ActiveModelBehavior for ActiveModel {}
