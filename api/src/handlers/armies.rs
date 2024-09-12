use armies_of_avalon_service::army_service::ArmyQuery;
use axum::{debug_handler, extract::State, Json};
use entity::armies::Model as ArmyModel;

use crate::{types::game_defaults::ArmyDefaults, utils::error::AppError, AppState};

#[debug_handler]
pub async fn get_all_armies(state: State<AppState>) -> Result<Json<Vec<ArmyDefaults>>, AppError> {
    let armies_models: Vec<ArmyModel> = ArmyQuery::get_all_armies(&state.conn).await?;

    let armies = armies_models
        .iter()
        .map(|model: &ArmyModel| model.clone().into())
        .collect::<Vec<ArmyDefaults>>();

    Ok(Json(armies))
}
