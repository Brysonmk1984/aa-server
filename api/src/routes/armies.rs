use axum::{routing::get, Router};

use crate::{handlers::armies::get_all_armies, AppState};

pub fn armies_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(get_all_armies))
        .with_state(state.clone())
}
