use armies_of_avalon_service::sea_orm::DatabaseConnection;
use axum::extract::{Path, State};

use crate::AppState;

pub struct NationController {}

impl NationController {
    // #[debug_handler]
    pub async fn buy_army(
        State(state): State<AppState>,
        Path((nation_id, army_id)): Path<(i32, i32)>,
    ) {
        println!("{nation_id} {army_id}");
    }
}
