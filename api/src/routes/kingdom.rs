use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handlers::kingdom::{buy_army, get_nation_and_armies},
    AppState,
};

pub fn kingdom_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/:nation_id/army/:army_id", post(buy_army))
        .route("/:user_id", get(get_nation_and_armies))
        .with_state(state.clone())
}
