// mod mutation;
// mod query;

// pub use mutation::*;
// pub use query::*;
#![allow(warnings)]
pub use sea_orm;

use ::entity::armies::Entity as Armies;
use ::entity::nation_armies::{Entity as NationArmies, Model as NationArmiesModel};
use ::entity::nations::{Entity as Nations, Model as NationsModel};
use sea_orm::*;

pub struct Query;

impl Query {
    pub async fn find_army_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<Option<<Armies as sea_orm::EntityTrait>::Model>, DbErr> {
        Armies::find_by_id(id).one(db).await
    }

    pub async fn get_all_armies(
        db: &DbConn,
    ) -> Result<Vec<<Armies as sea_orm::EntityTrait>::Model>, DbErr> {
        Armies::find().all(db).await
    }

    pub async fn get_nation_with_nation_armies(
        db: &DbConn,
        id: i32,
    ) -> Result<Vec<(NationsModel, Vec<NationArmiesModel>)>, DbErr> {
        let result = Nations::find_by_id(id)
            .find_with_related(NationArmies)
            .all(db)
            .await?;

        Ok(result)
    }
}
