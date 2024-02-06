//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "battles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub nation_id_east: i32,
    pub nation_id_west: i32,
    pub nation_campaign_level_id: Option<i32>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub outcome: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::nation_campaign_levels::Entity",
        from = "Column::NationCampaignLevelId",
        to = "super::nation_campaign_levels::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    NationCampaignLevels,
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

impl Related<super::nation_campaign_levels::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::NationCampaignLevels.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
