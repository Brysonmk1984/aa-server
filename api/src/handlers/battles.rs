use std::collections::HashMap;

use armies_of_avalon_service::Query;
use axum::{
    debug_handler,
    extract::{Json, State},
    http::StatusCode,
    Extension,
};
use serde::Deserialize;
use serde_json::Value;

use crate::{handlers::armies::get_all_armies, AppState};
use aa_battles::{
    do_battle,
    types::{Army, Nation, NationArmy},
};

#[derive(Deserialize, Debug)]
pub struct BattleCompetitors {
    pub east_competitor: i32,
    pub west_competitor: i32,
}

#[debug_handler]
pub async fn run_battle(
    state: State<AppState>,
    Extension(_claims): Extension<HashMap<String, Value>>,
    Json(body): Json<BattleCompetitors>,
) -> Result<
    //Json<Vec<(entity::nations::Model, Vec<entity::nation_armies::Model>)>>,
    (),
    (StatusCode, &'static str),
> {
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

    //println!("{nation_and_nation_armies_one:?} {nation_and_nation_armies_two:?}");
    let competitors = (east_tuple, west_tuple);
    let result = do_battle(army_defaults, competitors);

    println!("{:?}", result);

    //nation_and_nation_armies_one.append(&mut nation_and_nation_armies_two);

    let battle_result = entity::battles::Model {
        nation_id_east: east_nation.id,
        nation_id_west: west_nation.id,
        nation_campaign_level_id: None,
        ..Default::default()
    };

    Ok(())
}
