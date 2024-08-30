use axum::{routing::get, Router};

use crate::{handlers::game::get_game_data, AppState};

pub fn game_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(get_game_data))
        .with_state(state.clone())
}
