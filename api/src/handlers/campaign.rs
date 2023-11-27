use armies_of_avalon_service::Query;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use entity::{nation_armies::Model as NationArmiesModel, nations::Model as NationsModel};

use crate::AppState;

pub async fn get_all_campaign_nations_details(
    State(state): State<AppState>,
) -> Result<Json<Vec<(NationsModel, Vec<NationArmiesModel>)>>, (StatusCode, &'static str)> {
    Query::get_all_nations();
    todo!();
}

pub async fn get_campaign_nation_details(
    State(state): State<AppState>,
    Path(nation_id): Path<i32>,
) -> Result<Json<(NationsModel, Vec<NationArmiesModel>)>, (StatusCode, &'static str)> {
    let nation_and_armies =
        Query::get_nation_with_nation_armies_by_nation_id(&state.conn, nation_id)
            .await
            .expect(
                "A Nation and a vec of nation armies should return when fetching by nation id!",
            );
    dbg!(&nation_and_armies);
    Ok(Json(nation_and_armies))
}
