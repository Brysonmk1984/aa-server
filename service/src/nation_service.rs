#![allow(warnings)]

use sea_orm;
use std::collections::HashMap;
use std::fmt;

use ::entity::armies::{self, Entity as Armies, Model};
use ::entity::campaign_levels::{self, Entity as CampaignLevels, Model as CampaignLevelsModel};
use ::entity::nation_armies::{self, Entity as NationArmies, Model as NationArmiesModel};
use ::entity::nations::{self, Entity as Nations, Model as NationsModel};
use ::entity::users::{self, Column, Entity as Users, Model as UsersModel};
use sea_orm::sea_query::OnConflict;
use sea_orm::*;
use serde::Deserialize;
use strum::EnumString;

use crate::user_service;

#[derive(Deserialize)]
pub struct GetAllNationsParams {
    pub is_npc: Option<bool>,
}

pub struct NationQuery;
impl NationQuery {
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

    pub async fn get_campaign_nation_with_nation_armies_by_nation_id(
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
}

pub struct NationMutation;

impl NationMutation {
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
            nation_to_be_updated.update(db).await?;

            let matching_army_template = army_option.unwrap();

            let existing_army_of_same_type = NationArmies::find()
                .filter(nation_armies::Column::ArmyId.eq(army_id))
                .filter(nation_armies::Column::NationId.eq(nation_id))
                .one(db)
                .await?;

            let mut result;

            match existing_army_of_same_type {
                Some(nation_army) => {
                    let nation_army_to_be_inserted = nation_armies::ActiveModel {
                        count: Set(nation_army.count + matching_army_template.count),
                        ..nation_army.into()
                    };
                    result = nation_army_to_be_inserted.update(db).await;
                }
                None => {
                    println!("'IN NONE");
                    let nation_army_to_be_inserted = nation_armies::ActiveModel {
                        nation_id: Set(nation_id),
                        army_id: Set(army_id),
                        count: Set(matching_army_template.count),
                        army_name: Set(matching_army_template.name),
                        ..Default::default()
                    };
                    result = nation_army_to_be_inserted.insert(db).await;
                }
            };

            println!("{result:?}");

            match result {
                Ok(model) => Ok(model),
                Err(e) => {
                    dbg!(&e);
                    Err(e)
                }
            }
        }
    }

    pub async fn adjust_nation_army_counts(
        nation_id: i32,
        post_battle_nation_armies: Vec<NationArmiesModel>,
        db: &DbConn,
    ) -> Result<(), DbErr> {
        // 1. Create one vecs, one to hold ids to delete, and create a hashmap that holds ids to update and count
        let mut update_hash_map: HashMap<i32, i32> = HashMap::new();
        let mut delete_vec = vec![];

        // 2. Get all armies belonging to a nation
        let db_nation_armies = NationArmies::find()
            .filter(nation_armies::Column::NationId.eq(nation_id))
            .all(db)
            .await?;

        // 3. loop through each army in 'armies' vec
        post_battle_nation_armies.iter().for_each(|nation_army| {
            println!("{nation_army:?}");

            // 4. Compare count in DB to param value
            if (nation_army.count <= 0) {
                // 5. If count is zero, push id into delete array
                delete_vec.push(nation_army.id.clone());
            } else {
                // 6. If lower than db value AND gt zero, push nation id and count into map
                db_nation_armies.iter().for_each(|db_na| {
                    if (db_na.army_id == nation_army.army_id) {
                        if (nation_army.count < db_na.count) {
                            update_hash_map.insert(nation_army.id, nation_army.count);
                        }
                    }
                });
            }
        });

        if (delete_vec.len() > 0) {
            NationMutation::delete_vanquished_armies(delete_vec, db).await?;
        }

        if (update_hash_map.len() > 0) {
            NationMutation::update_partial_armies(update_hash_map, db).await?;
        }
        println!("FINISHED UPDATING");
        Ok(())
    }

    /**
     * DELETE: For each Battalion that's completely destroyed, delete the record from the DB
     */
    async fn delete_vanquished_armies(delete_vec: Vec<i32>, db: &DbConn) -> Result<(), DbErr> {
        println!("delete_vec: {delete_vec:?}");
        // 7. Delete dead battalions
        let delete_res: DeleteResult = NationArmies::delete_many()
            .filter(nation_armies::Column::Id.is_in(delete_vec))
            .exec(db)
            .await?;
        println!("delete_res: {delete_res:?}");

        Ok(())
    }

    /**
     * UPDATE: For each Battalion that suffered partial losses, update them in the DB
     */
    async fn update_partial_armies(
        update_hash_map: HashMap<i32, i32>,
        db: &DbConn,
    ) -> Result<(), DbErr> {
        // 8. Update partial battalions
        let hash_map_count = update_hash_map.len();

        let vec_of_hash: Vec<String> = update_hash_map
            .into_iter()
            .enumerate()
            .map(|(index, (k, v))| {
                if ((index + 1) == hash_map_count) {
                    format!("({k},{v})")
                } else {
                    format!("({k},{v}),")
                }
            })
            .collect();

        let values = vec_of_hash.join("");

        let sql = format!(
            "
    UPDATE nation_armies as nation_armies_table set
        id = temp.id,
        count = temp.count
    FROM (VALUES
       {values}
    ) as temp(id, count)
    WHERE nation_armies_table.id =  temp.id;
"
        );

        let statement = Statement::from_string(sea_orm::DatabaseBackend::Postgres, sql.to_owned());

        let update_res = db.execute_unprepared(sql.as_str()).await;

        println!("update_res: {update_res:?}");
        Ok(())
    }
}
