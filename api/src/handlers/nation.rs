use armies_of_avalon_service::{sea_orm::DatabaseConnection, Mutation};
use axum::{
    async_trait,
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use axum_macros::debug_handler;

use crate::AppState;

pub async fn buy_army(
    State(state): State<AppState>,
    Path((nation_id, army_id)): Path<(i32, i32)>,
) -> Result<
    //Json<Vec<(entity::nations::Model, Vec<entity::nation_armies::Model>)>>,
    Json<()>,
    (StatusCode, &'static str),
> {
    println!("{nation_id} {army_id}");

    let result = Mutation::buy_army(&state.conn, nation_id, army_id)
        .await
        .unwrap();

    println!("done! {result:?}");

    Ok(Json(result))
}
