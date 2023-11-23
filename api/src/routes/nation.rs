use std::sync::Arc;

use axum::{extract::State, routing::post, Router};

use crate::{handlers::nation::buy_army, AppState};

pub fn nation_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/:nation_id/army/:army_id", post(buy_army))
        .with_state(state.clone())
}
