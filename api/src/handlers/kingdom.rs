use armies_of_avalon_service::{Mutation, Query};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use axum_macros::debug_handler;
use entity::{nation_armies::Model as NationArmiesModel, nations::Model as NationsModel};

use crate::AppState;

#[debug_handler]
pub async fn get_nation_and_armies(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<Json<(NationsModel, Vec<NationArmiesModel>)>, (StatusCode, &'static str)> {
    let nation_and_armies = Query::get_nation_with_nation_armies_by_user_id(&state.conn, user_id)
        .await
        .expect("A Nation and a vec of nation armies should return!");
    dbg!(&nation_and_armies);
    Ok(Json(nation_and_armies))
}

#[debug_handler]
pub async fn buy_army(
    State(state): State<AppState>,
    Path((nation_id, army_id)): Path<(i32, i32)>,
) -> Result<Json<entity::nation_armies::Model>, (StatusCode, &'static str)> {
    println!("{nation_id} {army_id}");

    let result = Mutation::buy_army(&state.conn, nation_id, army_id)
        .await
        .unwrap();

    Ok(Json(result))
}
