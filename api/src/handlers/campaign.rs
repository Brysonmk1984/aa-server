use std::collections::HashMap;

use crate::{utils::error::AppError, AppState, Reward, CAMPAIGN_LEVEL_REWARDS_CELL};
use armies_of_avalon_service::{self, campaign_service::CampaignQuery};
use axum::{
    debug_handler,
    extract::{Path, State},
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
) -> Result<Json<(Vec<CampaignLevelsModel>, HashMap<i32, (i32, Reward)>)>, AppError> {
    let mut campaign_levels: Vec<CampaignLevelsModel> =
        CampaignQuery::get_all_campaign_levels(&state.conn).await?;
    campaign_levels.sort_by_key(|campaign_level| campaign_level.level);

    let rewards_map = CAMPAIGN_LEVEL_REWARDS_CELL.get().unwrap().clone();

    return Ok(Json((campaign_levels, rewards_map)));
}

#[derive(Debug, Serialize)]
pub struct NationWithArmies {
    nation_details: NationsModel,
    all_armies: Vec<NationArmiesModel>,
}

#[debug_handler]
pub async fn get_campaign_nation_details(
    State(state): State<AppState>,
    Path(nation_id): Path<i32>,
) -> Result<Json<NationWithArmies>, AppError> {
    let (nation_details, all_armies) =
        CampaignQuery::get_campaign_nation_with_nation_armies_by_nation_id(&state.conn, nation_id)
            .await?;

    let combined_nation_armies = NationWithArmies {
        nation_details,
        all_armies,
    };

    Ok(Json(combined_nation_armies))
}
