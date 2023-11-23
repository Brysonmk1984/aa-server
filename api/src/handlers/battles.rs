use armies_of_avalon_service::Query;
use axum::{extract::State, http::StatusCode, Json};
use axum_macros::debug_handler;
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize, Debug)]
pub struct BattleCompetitors {
    pub east_competitor: i32,
    pub west_competitor: i32,
}

#[debug_handler]
pub async fn run_battle(
    state: State<AppState>,
    Json(body): Json<BattleCompetitors>,
) -> Result<
    Json<Vec<(entity::nations::Model, Vec<entity::nation_armies::Model>)>>,
    (StatusCode, &'static str),
> {
    println!("{:?}", body);
    let mut nation_and_nation_armies_one =
        Query::get_nation_with_nation_armies(&state.conn, body.east_competitor)
            .await
            .expect("Cannot get nation with armies!");
    let mut nation_and_nation_armies_two =
        Query::get_nation_with_nation_armies(&state.conn, body.west_competitor)
            .await
            .expect("Cannot get nation with armies!");

    println!("{nation_and_nation_armies_one:?} {nation_and_nation_armies_two:?}");
    nation_and_nation_armies_one.append(&mut nation_and_nation_armies_two);

    Ok(Json(nation_and_nation_armies_one))
}
