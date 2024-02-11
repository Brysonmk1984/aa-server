use std::collections::HashMap;

use armies_of_avalon_service::{Mutation, Query};
use axum::{
    debug_handler,
    extract::{Path, State},
    Extension, Json,
};

use entity::{nation_armies::Model as NationArmiesModel, nations::Model as NationsModel};
use serde_json::Value;

use crate::{utils::error::AppError, AppState};

#[debug_handler]
pub async fn get_nation_and_armies_by_user_id(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Extension(_claims): Extension<HashMap<String, Value>>,
) -> Result<Json<(NationsModel, Vec<NationArmiesModel>)>, AppError> {
    // todo!("Verify that the user from the auth token is the one user from user_id");

    let nation_and_armies =
        Query::get_nation_with_nation_armies_by_user_id(&state.conn, user_id).await?;
    dbg!(&nation_and_armies);
    Ok(Json(nation_and_armies))
}

#[debug_handler]
pub async fn buy_army(
    State(state): State<AppState>,
    Path((nation_id, army_id)): Path<(i32, i32)>,
    Extension(_claims): Extension<HashMap<String, Value>>,
) -> Result<Json<entity::nation_armies::Model>, AppError> {
    println!("{nation_id} {army_id}");

    // todo!("Verify that the user from the auth token is the one buying the army");

    let result = Mutation::buy_army(&state.conn, nation_id, army_id).await?;

    Ok(Json(result))
}
