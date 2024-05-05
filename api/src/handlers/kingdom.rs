use std::collections::HashMap;

use armies_of_avalon_service::{
    army_service::ArmyQuery,
    nation_service::{NationMutation, NationQuery},
    types::types::ArmyNameForService,
};
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
        NationQuery::get_nation_with_nation_armies_by_user_id(&state.conn, user_id).await?;
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
    println!("ADS {nation_id} {army_id}");
    let result = NationMutation::buy_army(&state.conn, nation_id, army_id).await?;

    Ok(Json(result))
}

#[debug_handler]
pub async fn initialize_nation(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Extension(_claims): Extension<HashMap<String, Value>>,
) -> Result<Json<(NationsModel, Vec<NationArmiesModel>)>, AppError> {
    let created_nation = NationMutation::create_nation(user_id, &state.conn).await?;

    let militia = ArmyQuery::find_army_by_name(ArmyNameForService::MinuteMenMilitia, &state.conn)
        .await?
        .unwrap();

    let initial_nation_army_option = NationMutation::create_nation_army(
        created_nation.id,
        militia.id,
        militia.name,
        100,
        &state.conn,
    )
    .await;

    let initial_nation_army = initial_nation_army_option.unwrap();
    println!("{initial_nation_army:?}");
    let result = (created_nation, vec![initial_nation_army]);
    return Ok(Json(result));
}
