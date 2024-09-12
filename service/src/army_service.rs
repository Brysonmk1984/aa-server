#![allow(warnings)]
use sea_orm;
use std::fmt;

use ::entity::armies::{self, Entity as Armies, Model as ArmiesModel};

use sea_orm::*;
use serde::Deserialize;

use crate::types::types::ArmyNameForService;

#[derive(Deserialize)]
pub struct GetAllNationsParams {
    pub is_npc: Option<bool>,
}
pub struct ArmyQuery;
impl ArmyQuery {
    pub async fn find_army_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<Option<<Armies as sea_orm::EntityTrait>::Model>, DbErr> {
        Armies::find_by_id(id).one(db).await
    }

    pub async fn find_army_by_name(
        name: ArmyNameForService,
        db: &DbConn,
    ) -> Result<Option<<Armies as sea_orm::EntityTrait>::Model>, DbErr> {
        Armies::find()
            .filter(armies::Column::Name.eq(name.to_string()))
            .one(db)
            .await
    }

    pub async fn get_all_armies(db: &DbConn) -> Result<Vec<ArmiesModel>, DbErr> {
        Armies::find().all(db).await
    }
}
