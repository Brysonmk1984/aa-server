use armies_of_avalon_service::{self, GetAllNationsParams};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use axum_macros::debug_handler;
use entity::{nation_armies::Model as NationArmiesModel, nations::Model as NationsModel};

use crate::AppState;

#[debug_handler]
pub async fn get_all_campaign_nations_details(
    State(state): State<AppState>,
    Query(params): Query<GetAllNationsParams>,
) -> Result<Json<Vec<NationsModel>>, (StatusCode, &'static str)> {
    let nations: Vec<NationsModel> = armies_of_avalon_service::Query::get_all_nations(
        &state.conn,
        params,
    )
    .await
    .expect(
        "A vec of nations  should return when fetching with or without the is_npc query param!",
    );

    return Ok(Json(nations));
}

#[debug_handler]
pub async fn get_campaign_nation_details(
    State(state): State<AppState>,
    Path(nation_id): Path<i32>,
) -> Result<Json<(NationsModel, Vec<NationArmiesModel>)>, (StatusCode, &'static str)> {
    let nation_and_armies =
        armies_of_avalon_service::Query::get_nation_with_nation_armies_by_nation_id(
            &state.conn,
            nation_id,
        )
        .await
        .expect("A Nation and a vec of nation armies should return when fetching by nation id!");
    dbg!(&nation_and_armies);
    Ok(Json(nation_and_armies))
}
