#![allow(warnings)]
use std::{collections::HashMap, env};

use aa_battles::types::Belligerent;
use aa_battles::EndBattlePayload;
use armies_of_avalon_service::{
    battles_service::{self, BattleMutation},
    campaign_service::{CampaignMutation, CampaignQuery},
    nation_service::NationQuery,
};
use axum::{
    debug_handler,
    extract::{Json, Path, State},
    http::StatusCode,
    Extension,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use entity::battles::Model as BattlesModel;

use crate::handlers;
use crate::utils::error::AppError;
use crate::ARMY_DEFAULT_CELL;
use crate::WEAPON_ARMOR_CELL;
use crate::{handlers::armies::get_all_armies, AppState};
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

#[debug_handler]
pub async fn run_battle(
    state: State<AppState>,
    Path(level): Path<i32>,
    Json(body): Json<BattleCompetitors>,
) -> Result<Json<EndBattlePayload>, AppError> {
    println!("RUNNING BATTLE {level}");
    let result = get_all_armies(state.clone()).await?.0;

    // todo!("Verify that the nation retrieved belongs to the user from the auth token");

    //println!("{:?}", body);
    let (east_nation, east_nation_armies) =
        NationQuery::get_nation_with_nation_armies(&state.conn, body.east_competitor).await?;

    let east_tuple: (Nation, Vec<NationArmy>) = (
        east_nation.clone().into(),
        east_nation_armies
            .iter()
            .map(|army| army.clone().into())
            .collect::<Vec<NationArmy>>(),
    );

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
    // let EndBattlePayload {
    //     battle_result,
    //     headline,
    //     events,
    //     end_state,
    //     outcome,
    // } = do_battle(army_defaults, competitors)?;

    let game_defaults = GameDefaults {
        weapons_vs_armor: WEAPON_ARMOR_CELL.get().unwrap(),
        army_defaults: ARMY_DEFAULT_CELL.get().unwrap(),
        environment: env::var("ENVIRONMENT").unwrap(),
    };

    let end_battle_payload = do_battle(game_defaults, competitors)?;

    let campaign_level =
        CampaignQuery::get_campaign_level_by_level_number(&state.conn, level).await?;

    let completed_level = end_battle_payload.battle_result.winner == Some(Belligerent::EasternArmy);
    let winner = if completed_level {
        east_nation.id
    } else {
        west_nation.id
    };

    let campaign_nation_level_result = CampaignMutation::upsert_nation_campaign_level(
        &state.conn,
        east_nation.id,
        campaign_level.id,
        east_nation.name,
        level,
        completed_level,
    )
    .await?;

    println!("{campaign_nation_level_result:?}");

    let battle_record_result = BattleMutation::insert_battle_record(
        &state.conn,
        east_nation.id,
        west_nation.id,
        Some(campaign_nation_level_result.id),
        winner,
    )
    .await?;

    println!("{:?}", end_battle_payload.battle_result);

    let setting = BattlesModel {
        nation_id_east: east_nation.id,
        nation_id_west: west_nation.id,
        nation_campaign_level_id: Some(campaign_nation_level_result.id),
        ..Default::default()
    };

    // let battle_stats = BattleStats {
    //     setting,
    //     outcome: end_battle_payload,
    // };

    //println!("{battle_stats:?}");

    Ok(Json(end_battle_payload))
}
