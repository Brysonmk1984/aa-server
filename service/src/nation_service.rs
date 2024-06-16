#![allow(warnings)]

use anyhow::Result;
use sea_orm;
use std::collections::HashMap;
use std::fmt;

use crate::types;
use crate::types::types::ArmyNameForService;
use crate::user_service;
use ::entity::armies::{self, ActiveModel, Entity as Armies, Model};
use ::entity::campaign_levels::{self, Entity as CampaignLevels, Model as CampaignLevelsModel};
use ::entity::nation_armies::{
    self, ActiveModel as NationArmiesActiveModel, Entity as NationArmies,
    Model as NationArmiesModel,
};
use ::entity::nations::{
    self, ActiveModel as NationsActiveModel, Entity as Nations, Model as NationsModel,
};
use ::entity::users::{self, Column, Entity as Users, Model as UsersModel};

use sea_orm::sea_query::OnConflict;
use sea_orm::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetAllNationsParams {
    pub is_npc: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct PatchNationPayload {
    pub name: Option<String>,
    pub lore: Option<String>,
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

        let nation_id = &nation.clone().unwrap().id;
        let nation_armies = NationArmies::find()
            .filter(nation_armies::Column::NationId.eq(*nation_id))
            .all(db)
            .await?;

        Ok((nation.unwrap(), nation_armies))
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
    pub async fn create_nation(user_id: i32, db: &DbConn) -> Result<NationsModel, DbErr> {
        let nation_to_be_inserted = NationsActiveModel {
            user_id: Set(Some(user_id)),
            gold: Set(1000),
            is_npc: Set(false),
            ..Default::default()
        };

        let nation = nation_to_be_inserted.insert(db).await?;

        Ok(nation)
    }

    pub async fn patch_nation(
        nation_id: i32,
        db: &DbConn,
        payload: PatchNationPayload,
    ) -> Result<NationsModel, DbErr> {
        let the_result = Nations::find()
            .filter(nations::Column::Id.eq(nation_id))
            .one(db)
            .await;

        let nation_option: Option<NationsModel> = the_result.unwrap();

        match nation_option {
            Some(nation) => {
                let nation_to_be_inserted = NationsActiveModel {
                    id: Unchanged(nation.id),
                    name: Set(payload.name),
                    lore: Set(payload.lore),
                    ..Default::default()
                };

                let nation = nation_to_be_inserted.update(db).await?;
                Ok(nation)
            }
            None => Err(DbErr::RecordNotFound(format!("nation_id:{nation_id}"))),
        }
    }

    pub async fn create_nation_army(
        nation_id: i32,
        army_id: i32,
        army_name: String,
        count: i32,
        db: &DbConn,
    ) -> Result<NationArmiesModel, DbErr> {
        let nation_army_to_be_inserted = nation_armies::ActiveModel {
            nation_id: Set(nation_id),
            army_id: Set(army_id),
            army_name: Set(army_name),
            count: Set(count),
            ..Default::default()
        };

        let created_nation_army: NationArmiesModel = nation_army_to_be_inserted.insert(db).await?;

        Ok(created_nation_army)
    }

    pub async fn update_gold_from_income_timer(db: &DbConn) -> Result<(), DbErr> {
        let sql = "
            SELECT nations.id, name, gold, MAX(level) AS max_level
            FROM nations 
            INNER JOIN nation_campaign_levels
                ON nations.id = nation_campaign_levels.nation_id
            WHERE completed = TRUE
            GROUP BY nations.id;
        ";

        let statement = Statement::from_string(sea_orm::DatabaseBackend::Postgres, sql.to_owned());
        let query_res_vec = db.query_all(statement).await;

        let res = query_res_vec.unwrap();
        let update_hash_map = res.iter().fold(HashMap::new(), |mut acc, cur| {
            let id = cur.try_get::<i32>("", "id").unwrap();
            let gold = cur.try_get::<i32>("", "gold").unwrap();
            let level = cur.try_get::<i32>("", "max_level").unwrap();
            let updated_gold = gold + 100 + (level * 10);
            acc.insert(id, updated_gold);
            acc
        });

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
    UPDATE nations set
        id = temp.id,
        gold = temp.gold
    FROM (VALUES
       {values}
    ) as temp(id, gold)
    WHERE nations.id =  temp.id;
"
        );
        let statement = Statement::from_string(sea_orm::DatabaseBackend::Postgres, sql.to_owned());

        let update_res = db.execute_unprepared(sql.as_str()).await;
        println!("{update_res:?}");
        Ok(())
    }

    pub async fn update_gold_from_upkeep_timer(db: &DbConn) -> Result<(), DbErr> {
        // 1. Get required data from DB
        // // get all non-campaign nations from db - nations table
        // // get total nation_army army count - nation_armies table
        // // create a custom join and a custom struct with the new data

        let sql = "
            SELECT nations.id, gold, SUM(nation_armies.count) as total_army_count 
            FROM nations
            JOIN nation_armies
                ON nations.id = nation_armies.nation_id
            WHERE nations.is_npc = FALSE
            GROUP BY nations.id;
        ";

        let statement = Statement::from_string(sea_orm::DatabaseBackend::Postgres, sql.to_owned());
        let query_res_vec = db.query_all(statement).await;

        let res = query_res_vec.unwrap();

        // 2. loop through each nation w/ details
        // // Calculate their upkeep by seeing which upkeep bracket they fall under
        // // // none - below 10k soldiers - 0 gold
        // // // low - over 10k soldiers - 25 gold
        // // // medium - over 50k soldiers - 75 gold
        // // // high - over 90k soldiers - 150 gold
        // // subtract upkeep amount from nation's gold
        // create new hash of nation id to income
        // do a bulk update from hash

        let update_hash_map = res.iter().fold(HashMap::new(), |mut acc, cur| {
            let id = cur.try_get::<i32>("", "id").unwrap();
            let gold = cur.try_get::<i32>("", "gold").unwrap();
            let total_army_count = cur.try_get::<i64>("", "total_army_count").unwrap();

            let upkeep_cost = if (total_army_count > 90_000) {
                150
            } else if (total_army_count > 50_000) {
                75
            } else if (total_army_count > 10_000) {
                25
            } else {
                0
            };

            let potentially_negative_gold = gold - upkeep_cost;
            let updated_gold = if (potentially_negative_gold < 0) {
                0
            } else {
                potentially_negative_gold
            };
            acc.insert(id, updated_gold);
            acc
        });

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
    UPDATE nations set
        id = temp.id,
        gold = temp.gold
    FROM (VALUES
       {values}
    ) as temp(id, gold)
    WHERE nations.id =  temp.id;
"
        );
        let statement = Statement::from_string(sea_orm::DatabaseBackend::Postgres, sql.to_owned());

        let update_res = db.execute_unprepared(sql.as_str()).await;

        println!("UPKEEP timer! ");
        Ok(())
    }

    pub async fn update_gold(db: &DbConn, nation_id: i32, gold: i32) -> Result<(), DbErr> {
        let sql = format!(
            "
            UPDATE nations
            SET gold = (gold + 11)
            WHERE id = {nation_id};
            "
        );

        let statement = Statement::from_string(sea_orm::DatabaseBackend::Postgres, sql.to_owned());

        let update_res = db.execute_unprepared(sql.as_str()).await?;
        Ok(())
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

    pub async fn upsert_nation_army(
        db: &DbConn,
        nation_id: i32,
        army_name: ArmyNameForService,
        count: i32,
    ) -> Result<(), DbErr> {
        let army_option = Armies::find()
            .filter(armies::Column::Name.eq(army_name.to_string()))
            .one(db)
            .await?;

        let army_id = army_option.unwrap().id;

        let existing_army_of_same_type = NationArmies::find()
            .filter(nation_armies::Column::ArmyId.eq(army_id))
            .filter(nation_armies::Column::NationId.eq(nation_id))
            .one(db)
            .await?;

        let mut result;

        match existing_army_of_same_type {
            Some(nation_army) => {
                let nation_army_to_be_inserted = nation_armies::ActiveModel {
                    count: Set(nation_army.count + count),
                    ..nation_army.into()
                };
                result = nation_army_to_be_inserted.update(db).await?;
            }
            None => {
                let nation_army_to_be_inserted = nation_armies::ActiveModel {
                    nation_id: Set(nation_id),
                    army_id: Set(army_id),
                    count: Set(count),
                    army_name: Set(army_name.to_string()),
                    ..Default::default()
                };
                result = nation_army_to_be_inserted.insert(db).await?;
            }
        }
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
