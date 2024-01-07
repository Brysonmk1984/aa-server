use armies_of_avalon_service::Query;
use axum::{debug_handler, extract::State, http::StatusCode, Json};

use entity::armies::Model as ArmiesModel;

use crate::AppState;

#[debug_handler]
pub async fn get_all_armies(
    state: State<AppState>,
) -> Result<Json<Vec<ArmiesModel>>, (StatusCode, &'static str)> {
    let armies = Query::get_all_armies(&state.conn)
        .await
        .expect("Cannot get all armies!");

    Ok(Json(armies))
}
