#![allow(warnings)]

use std::fmt;

pub use sea_orm;

use ::entity::armies::{self, Entity as Armies, Model};
use ::entity::campaign_levels::{self, Entity as CampaignLevels, Model as CampaignLevelsModel};
use ::entity::nation_armies::{self, Entity as NationArmies, Model as NationArmiesModel};
use ::entity::nations::{self, Entity as Nations, Model as NationsModel};
use ::entity::users::{self, Column, Entity as Users, Model as UsersModel};
use sea_orm::sea_query::OnConflict;
use sea_orm::*;
use serde::Deserialize;
use strum::EnumString;
pub struct Query;

pub mod battles_service;
pub mod campaign_service;
pub mod cron_service;
pub mod external_requests;
pub mod initialization_service;

#[derive(Deserialize)]
pub struct GetAllNationsParams {
    pub is_npc: Option<bool>,
}

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
    ) -> Result<(NationsModel, Vec<NationArmiesModel>), DbErr> {
        let result = Nations::find_by_id(id)
            .find_with_related(NationArmies)
            .all(db)
            .await?;
        let result: (NationsModel, Vec<NationArmiesModel>) = result.get(0).unwrap().clone();
        Ok(result)
    }

    pub async fn get_all_nations(
        db: &DbConn,
        params: GetAllNationsParams,
    ) -> Result<Vec<NationsModel>, DbErr> {
        let result;

        match params.is_npc {
            Some(is_npc) => {
                result = Nations::find()
                    .filter(nations::Column::IsNpc.eq(is_npc))
                    .all(db)
                    .await?;
            }
            None => {
                result = Nations::find().all(db).await?;
            }
        }

        Ok(result)
    }

    pub async fn get_nation_with_nation_armies_by_user_id(
        db: &DbConn,
        user_id: i32,
    ) -> Result<(NationsModel, Vec<NationArmiesModel>), DbErr> {
        let nation = Nations::find()
            .filter(nations::Column::UserId.eq(user_id))
            .one(db)
            .await?;

        if nation.is_none() {
            panic!("No nation for user {}", user_id)
        }

        let nation_id = &nation.clone().unwrap().id;
        let nation_armies = NationArmies::find()
            .filter(nation_armies::Column::NationId.eq(*nation_id))
            .all(db)
            .await;

        match nation_armies {
            Ok(n_armies) => Ok((nation.unwrap(), n_armies)),
            Err(_) => Ok((nation.unwrap(), vec![])),
        }
    }

    pub async fn get_nation_with_nation_armies_by_nation_id(
        db: &DbConn,
        level: i32,
    ) -> Result<(NationsModel, Vec<NationArmiesModel>), DbErr> {
        println!("{level}");
        let campaign_level = CampaignLevels::find()
            .filter(campaign_levels::Column::Level.eq(level))
            .one(db)
            .await?;
        if campaign_level.is_none() {
            panic!("No campaign level: {}", level)
        }
        let level_nation_id = campaign_level.unwrap().nation_id.clone();
        let nation = Nations::find()
            .filter(nations::Column::Id.eq(level_nation_id))
            .one(db)
            .await?;
        println!("{nation:?}");
        if nation.is_none() {
            panic!("No nation with id: {}", level_nation_id)
        }

        let nation_armies = NationArmies::find()
            .filter(nation_armies::Column::NationId.eq(level_nation_id))
            .all(db)
            .await;
        println!("{nation_armies:?}");
        match nation_armies {
            Ok(n_armies) => Ok((nation.unwrap(), n_armies)),
            Err(_) => Ok((nation.unwrap(), vec![])),
        }
    }

    pub async fn get_all_campaign_levels(db: &DbConn) -> Result<Vec<CampaignLevelsModel>, DbErr> {
        let campaign_levels: Vec<CampaignLevelsModel> = CampaignLevels::find().all(db).await?;
        Ok(campaign_levels)
    }

    pub async fn get_campaign_level_id_by_level_number(
        db: &DbConn,
        level: i32,
    ) -> Result<i32, DbErr> {
        let campaign_levels: Result<std::option::Option<CampaignLevelsModel>, DbErr> =
            CampaignLevels::find()
                .filter(campaign_levels::Column::Level.eq(level))
                .one(db)
                .await;

        let campaign_level: CampaignLevelsModel = campaign_levels.unwrap().unwrap();
        Ok(campaign_level.id)
    }
}

pub struct Mutation;

#[derive(Debug, Clone, Deserialize)]
pub struct Auth0UserPart {
    pub email: String,
    pub email_verified: bool,
    pub auth0_sub: String,
}

impl Mutation {
    pub async fn insert_or_return_user(
        db: &DbConn,
        partial_user: Auth0UserPart,
    ) -> Result<UsersModel, DbErr> {
        let user = users::ActiveModel {
            email: Set(partial_user.email.to_owned()),
            email_verified: Set(partial_user.email_verified.to_owned()),
            auth0_sub: Set(partial_user.auth0_sub.to_owned()),
            ..Default::default()
        };

        let result = user.clone().insert(db).await;

        match result {
            Ok(u) => {
                println!("{u:#?}");
                Ok(u.try_into_model().unwrap() as UsersModel)
            }
            Err(error) => {
                let user = Users::find()
                    .filter(users::Column::Auth0Sub.eq(&partial_user.auth0_sub))
                    .one(db)
                    .await?;

                Ok(user.unwrap() as UsersModel)
            }
        }
    }

    pub async fn buy_army(
        db: &DbConn,
        nation_id: i32,
        army_id: i32,
    ) -> Result<nation_armies::Model, DbErr> {
        let nation_option: Option<nations::Model> = Nations::find_by_id(nation_id).one(db).await?;
        let army_option: Option<armies::Model> = Armies::find_by_id(army_id).one(db).await?;
        let army_cost;
        let nation_gold;

        if let Some(army) = &army_option {
            army_cost = army.cost;
        } else {
            let error_message = format!("army_id: {army_id}");
            //return DbErr::RecordNotFound(error_message);
            return Err(DbErr::Custom(error_message));
        }

        if let Some(nation) = &nation_option {
            nation_gold = nation.gold;
        } else {
            let error_message = format!("nation_id: {nation_id}");
            //return Err(DbErr::RecordNotFound(error_message));
            return Err(DbErr::Custom(error_message));
        }

        if (army_cost > nation_gold) {
            let error_message = format!("Nation does not have enough Gold!");
            return Err(DbErr::Custom(error_message));
        } else {
            // Now we create a transaction that:
            // first subtracts the gold from the nation
            // second creates the nation_army

            // <Fn, A, B> -> Result<A, B>
            // let transaction_result = db
            //     .transaction::<_, (), DbErr>(|txn| {
            //         Box::pin(async move {
            //             nations::ActiveModel {
            //                 gold: Set(nation_gold - army_cost),
            //                 ..Default::default()
            //             }
            //             .save(txn)
            //             .await?;

            //             let matching_army = army_option.unwrap();

            //             let nation_army_to_be_inserted = nation_armies::ActiveModel {
            //                 nation_id: Set(nation_id),
            //                 army_id: Set(army_id),
            //                 count: Set(matching_army.count),
            //                 army_name: Set(matching_army.name),
            //                 ..Default::default()
            //             };

            //             nation_army_to_be_inserted.insert(db).await?;

            //             Ok(())
            //         })
            //     })
            //     .await;

            // match transaction_result {
            //     Ok(_) => Ok(()),

            //     Err(error) => Err(anyhow!(error)),
            // }

            let mut nation_to_be_updated: nations::ActiveModel = nation_option.unwrap().into();
            nation_to_be_updated.gold = Set(nation_gold - army_cost);
            let pear: nations::Model = nation_to_be_updated.update(db).await?;

            let matching_army = army_option.unwrap();

            let nation_army_to_be_inserted = nation_armies::ActiveModel {
                nation_id: Set(nation_id),
                army_id: Set(army_id),
                count: Set(matching_army.count),
                army_name: Set(matching_army.name),
                ..Default::default()
            };

            let result = nation_army_to_be_inserted.insert(db).await?;

            Ok(result)
        }
    }
}
