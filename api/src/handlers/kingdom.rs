use std::collections::HashMap;

use armies_of_avalon_service::{
    army_service::ArmyQuery,
    nation_service::{NationMutation, NationQuery, PatchNationPayload},
    types::types::ArmyNameForService,
};
use axum::{
    debug_handler,
    extract::{Path, State},
    Extension, Json,
};

use entity::{nation_armies::Model as NationArmiesModel, nations::Model as NationsModel};
use serde::Deserialize;
use serde_json::Value;

use crate::{utils::error::AppError, AppState};

#[debug_handler]
pub async fn get_nation_and_armies_by_user_id(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Extension(_claims): Extension<HashMap<String, Value>>,
) -> Result<Json<(NationsModel, Vec<NationArmiesModel>)>, AppError> {
    // todo!("Verify that the user from the auth token is the one user from user_id");
    println!("CLAIMS {_claims:?}");
    let nation_and_armies =
        NationQuery::get_nation_with_nation_armies_by_user_id(&state.conn, user_id).await?;
    //dbg!(&nation_and_armies);
    Ok(Json(nation_and_armies))
}

#[debug_handler]
pub async fn get_nation_armies_by_nation_id(
    State(state): State<AppState>,
    Path((_, nation_id)): Path<(i32, i32)>,
    Extension(_claims): Extension<HashMap<String, Value>>,
) -> Result<Json<Vec<NationArmiesModel>>, AppError> {
    let nation_armies = NationQuery::get_nation_armies_by_nation_id(&state.conn, nation_id).await?;
    //dbg!(&nation_armies);
    Ok(Json(nation_armies))
}

#[derive(Deserialize)]
pub struct BuyArmyPayload {
    pub quantity: i32,
}

#[debug_handler]
pub async fn buy_army(
    State(state): State<AppState>,
    Path((_user_id, nation_id, army_id)): Path<(i32, i32, i32)>,
    Extension(_claims): Extension<HashMap<String, Value>>,
    Json(payload): Json<BuyArmyPayload>,
) -> Result<Json<(entity::nation_armies::Model, i32)>, AppError> {
    println!("{nation_id} {army_id}");

    // todo!("Verify that the user from the auth token is the one buying the army");
    println!("BUYING:{nation_id} {army_id}");
    let result =
        NationMutation::buy_army(&state.conn, nation_id, army_id, payload.quantity).await?;

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

#[debug_handler]
pub async fn patch_nation(
    State(state): State<AppState>,
    Path((_user_id, nation_id)): Path<(i32, i32)>,
    Json(payload): Json<PatchNationPayload>,
) -> Result<Json<NationsModel>, AppError> {
    let nation = NationMutation::patch_nation(nation_id, &state.conn, payload).await?;

    return Ok(Json(nation));
}
