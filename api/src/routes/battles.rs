use crate::{handlers::battles::run_battle, AppState};
use axum::{routing::post, Router};

pub fn battles_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/campaign/levels/:level", post(run_battle))
        .with_state(state.clone())
}
