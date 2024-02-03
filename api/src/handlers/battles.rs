use std::collections::HashMap;

use armies_of_avalon_service::battles_service;
use armies_of_avalon_service::Query;
use axum::{
    debug_handler,
    extract::{Json, Path, State},
    http::StatusCode,
    Extension,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use entity::battles::Model as BattlesModel;

use crate::{handlers::armies::get_all_armies, AppState};
use aa_battles::{
    do_battle,
    types::{Army, BattleResult, Nation, NationArmy},
};

#[derive(Deserialize, Debug)]
pub struct BattleCompetitors {
    pub east_competitor: i32,
    pub west_competitor: i32,
}

#[derive(Serialize, Debug)]
pub struct BattleStats {
    setting: BattlesModel,
    outcome: String,
}

#[debug_handler]
pub async fn run_battle(
    state: State<AppState>,
    //Extension(_claims): Extension<HashMap<String, Value>>,
    Path(level): Path<i32>,
    Json(body): Json<BattleCompetitors>,
) -> Result<
    //Json<Vec<(entity::nations::Model, Vec<entity::nation_armies::Model>)>>,
    Json<BattleStats>,
    (StatusCode, &'static str),
> {
    println!("RUNNING BATTLE {level}");
    let result = get_all_armies(state.clone()).await?.0;
    let mut army_defaults = result
        .iter()
        .map(|army| army.clone().into())
        .collect::<Vec<Army>>();

    army_defaults.sort_by(|a, b| a.id.cmp(&b.id));

    // todo!("Verify that the nation retrieved belongs to the user from the auth token");

    //println!("{:?}", body);
    let (east_nation, east_nation_armies) =
        Query::get_nation_with_nation_armies(&state.conn, body.east_competitor)
            .await
            .expect("Cannot get nation with armies!");

    let east_tuple: (Nation, Vec<NationArmy>) = (
        east_nation.clone().into(),
        east_nation_armies
            .iter()
            .map(|army| army.clone().into())
            .collect::<Vec<NationArmy>>(),
    );

    let (west_nation, west_nation_armies) =
        Query::get_nation_with_nation_armies(&state.conn, body.west_competitor)
            .await
            .expect("Cannot get nation with armies!");

    let west_tuple: (Nation, Vec<NationArmy>) = (
        west_nation.clone().into(),
        west_nation_armies
            .iter()
            .map(|army| army.clone().into())
            .collect::<Vec<NationArmy>>(),
    );

    let competitors = (east_tuple, west_tuple);
    let outcome = do_battle(army_defaults, competitors);

    // @todo add record for nation_campaign_levels

    let battle_record_result = armies_of_avalon_service::Mutation::insert_battle_record(
        &state.conn,
        east_nation.id,
        west_nation.id,
        Some(level),
    )
    .await
    .expect("Cannot insert battle record!");

    let setting = BattlesModel {
        nation_id_east: east_nation.id,
        nation_id_west: west_nation.id,
        nation_campaign_level_id: None,
        ..Default::default()
    };

    let battle_stats = BattleStats { setting, outcome };

    println!("{battle_stats:?}");

    Ok(Json(battle_stats))
}
