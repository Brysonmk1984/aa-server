#![allow(warnings)]
use std::fmt;
use std::str::FromStr;
use std::{collections::HashMap, env};

use ::entity::nation_armies::{self, Entity as NationArmies, Model as NationArmiesModel};
use aa_battles::types::{ArmyName, BattleArmy, Belligerent, EndingBattalionStats};
use aa_battles::util::{create_hash_of_defaults, Stats};
use aa_battles::EndBattlePayload;
use armies_of_avalon_service::types::types::ArmyNameForService;
use armies_of_avalon_service::{
    battles_service::{self, BattleMutation},
    campaign_service::{CampaignMutation, CampaignQuery},
    nation_service::{NationMutation, NationQuery},
    types,
};
use axum::extract::Query;

use axum::{
    debug_handler,
    extract::{Json, Path, State},
    http::StatusCode,
    Extension,
};
use entity::battles::Model as BattlesModel;
use entity::nation_armies::Model;
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::utils::error::AppError;
use crate::AOE_SPREAD_CELL;
use crate::ARMY_DEFAULT_CELL;
use crate::CAMPAIGN_LEVEL_REWARDS_CELL;
use crate::WEAPON_ARMOR_CELL;
use crate::{AppState, Reward};

use aa_battles::{
    do_battle,
    types::{Army, BattleResult, GameDefaults, Nation, NationArmy},
};

#[derive(Deserialize, Debug)]
pub struct BattleCompetitors {
    pub east_competitor: i32,
    pub west_competitor: i32,
}

#[derive(Serialize, Debug)]
pub struct BattleStats {
    setting: BattlesModel,
    outcome: EndBattlePayload,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    disableCountAdjustment: Option<bool>,
}

#[debug_handler]
pub async fn run_battle(
    state: State<AppState>,
    Path(level): Path<i32>,
    Query(query): Query<QueryParams>,
    Json(body): Json<BattleCompetitors>,
) -> Result<Json<FrontEndPayload>, AppError> {
    println!("RUNNING BATTLE {level}");

    let mapped_armies = ARMY_DEFAULT_CELL
        .get()
        .unwrap()
        .iter()
        .map(|army_default| army_default.army.clone())
        .collect();
    let army_default_hash = create_hash_of_defaults(mapped_armies);

    let (east_nation, east_nation_armies) =
        NationQuery::get_nation_with_nation_armies(&state.conn, body.east_competitor).await?;

    let east_tuple: (Nation, Vec<NationArmy>) = (
        east_nation.clone().into(),
        east_nation_armies
            .iter()
            .map(|army| army.clone().into())
            .collect::<Vec<NationArmy>>(),
    );
    println!("east_tupleeast_tuple - {east_tuple:?}");
    let (west_nation, west_nation_armies) =
        NationQuery::get_nation_with_nation_armies(&state.conn, body.west_competitor).await?;

    let west_tuple: (Nation, Vec<NationArmy>) = (
        west_nation.clone().into(),
        west_nation_armies
            .iter()
            .map(|army| army.clone().into())
            .collect::<Vec<NationArmy>>(),
    );

    let competitors = (east_tuple, west_tuple);

    let game_defaults = GameDefaults {
        weapons_vs_armor: WEAPON_ARMOR_CELL.get().unwrap().clone(),
        aoe_vs_spread: AOE_SPREAD_CELL.get().unwrap().clone(),
        army_defaults: army_default_hash,
        environment: env::var("ENVIRONMENT").unwrap(),
    };

    let mut end_battle_payload = do_battle(game_defaults, competitors.clone())?;

    let campaign_level =
        CampaignQuery::get_campaign_level_by_level_number(&state.conn, level).await?;

    let completed_level = end_battle_payload.battle_result.winner == Some(Belligerent::EasternArmy);
    let mut reward: Option<(i32, Reward)> = None;

    let winner = if completed_level {
        // determine reward
        reward = Some(determine_reward(&level));
        let (reward_amount, reward_type) = reward.unwrap();
        // affects nation_armies and nation domain
        match reward_type {
            Reward::Gold => {
                NationMutation::update_gold(&state.conn, east_nation.id, reward_amount).await?;
            }
            Reward::Enlist(army) => {
                NationMutation::upsert_nation_army(
                    &state.conn,
                    east_nation.id,
                    ArmyNameForService::from_str(&army.to_string()).unwrap(),
                    reward_amount,
                )
                .await?;
            }
        };

        east_nation.id
    } else {
        println!("western win");
        west_nation.id
    };

    // affects campaign_level domain
    let campaign_nation_level_result = CampaignMutation::upsert_nation_campaign_level(
        &state.conn,
        east_nation.id,
        campaign_level.id,
        east_nation.name.unwrap_or_else(|| "".to_string()),
        level,
        completed_level,
    )
    .await?;

    let battle_record_result = BattleMutation::insert_battle_record(
        &state.conn,
        east_nation.id,
        west_nation.id,
        Some(campaign_nation_level_result.id),
        winner,
    )
    .await?;

    let cloned_armies = end_battle_payload.battle_result.eastern_battalions.clone();

    // TODO: When we support non-campaign battles, need to also update the western nation's counts
    let eastern_battalions = competitors.0 .1;
    let vec_post_battle_eastern_army_values: Vec<NationArmiesModel> = eastern_battalions
        .iter()
        .map(|nation_army| {
            let count = cloned_armies
                .iter()
                .fold(nation_army.count, |mut count, battalion| {
                    if (battalion.name == nation_army.army_name) {
                        battalion.count
                    } else {
                        count
                    }
                });

            NationArmiesModel {
                count,
                id: nation_army.id,
                army_id: nation_army.army_id,
                ..Default::default()
            }
        })
        .collect();
    println!("vec_post_battle_nation_army_values: {vec_post_battle_eastern_army_values:?}");

    // Disable updating of armies after battle if "disableCountAdjustment" query param is true
    if (!query.disableCountAdjustment.unwrap_or_else(|| false)) {
        NationMutation::adjust_nation_army_counts(
            east_nation.id,
            vec_post_battle_eastern_army_values,
            &state.conn,
        )
        .await?;
    }

    let setting = BattlesModel {
        nation_id_east: east_nation.id,
        nation_id_west: west_nation.id,
        nation_campaign_level_id: Some(campaign_nation_level_result.id),
        ..Default::default()
    };

    let front_end_payload = FrontEndPayload {
        reward,
        ..end_battle_payload.into()
    };
    Ok(Json(front_end_payload))
}

fn determine_reward(level: &i32) -> (i32, Reward) {
    let rewards_map = CAMPAIGN_LEVEL_REWARDS_CELL.get().unwrap();

    let result = *rewards_map.get(level).unwrap();

    result.clone()
}

#[derive(Serialize, Debug)]
pub struct FrontEndPayload {
    pub battle_result: BattleResult,
    pub army_compositions: (BattleArmy, BattleArmy),
    pub events: Vec<String>,
    pub stats: (Stats, Stats),
    pub reward: Option<(i32, Reward)>,
}
impl From<EndBattlePayload> for FrontEndPayload {
    fn from(end_battle_payload: EndBattlePayload) -> Self {
        Self {
            battle_result: end_battle_payload.battle_result,
            army_compositions: end_battle_payload.army_compositions,
            events: end_battle_payload.events,
            stats: end_battle_payload.stats,
            reward: None,
        }
    }
}

/// Serde deserialization decorator to map empty Strings to None,
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}
