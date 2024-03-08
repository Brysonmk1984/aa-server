use armies_of_avalon_service::army_service::ArmyQuery;
use axum::{debug_handler, extract::State, Json};

use entity::armies::Model as ArmiesModel;

use crate::{utils::error::AppError, AppState};

#[debug_handler]
pub async fn get_all_armies(state: State<AppState>) -> Result<Json<Vec<ArmiesModel>>, AppError> {
    let armies = ArmyQuery::get_all_armies(&state.conn).await?;

    Ok(Json(armies))
}
