use crate::AppState;
use armies_of_avalon_service::{self};
use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use entity::{
    campaign_levels::Model as CampaignLevelsModel, nation_armies::Model as NationArmiesModel,
    nations::Model as NationsModel,
};
use serde::Serialize;

#[debug_handler]
pub async fn get_all_campaign_levels(
    State(state): State<AppState>,
) -> Result<Json<Vec<CampaignLevelsModel>>, (StatusCode, &'static str)> {
    let mut campaign_levels: Vec<CampaignLevelsModel> =
        armies_of_avalon_service::Query::get_all_campaign_levels(&state.conn)
            .await
            .expect("A vec of campaign levels should be returned");
    campaign_levels.sort_by_key(|campaign_level| campaign_level.level);
    return Ok(Json(campaign_levels));
}

#[derive(Serialize)]
pub struct NationWithArmies {
    nation_details: NationsModel,
    all_armies: Vec<NationArmiesModel>,
}

#[debug_handler]
pub async fn get_campaign_nation_details(
    State(state): State<AppState>,
    Path(nation_id): Path<i32>,
) -> Result<Json<NationWithArmies>, (StatusCode, &'static str)> {
    let (nation_details, all_armies) =
        armies_of_avalon_service::Query::get_nation_with_nation_armies_by_nation_id(
            &state.conn,
            nation_id,
        )
        .await
        .expect("A Nation and a vec of nation armies should return when fetching by nation id!");

    let combined_nation_armies = NationWithArmies {
        nation_details,
        all_armies,
    };
    Ok(Json(combined_nation_armies))
}
