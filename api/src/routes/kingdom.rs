use crate::{
    handlers::kingdom::{
        buy_army, get_nation_and_armies_by_user_id, initialize_nation, patch_nation,
    },
    AppState,
};
use axum::{
    routing::{get, patch, post},
    Router,
};

pub fn kingdom_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/:user_id/nation/:nation_id/army/:army_id", post(buy_army))
        .route("/:user_id", get(get_nation_and_armies_by_user_id))
        .route("/:user_id", post(initialize_nation))
        .route("/:user_id/nation/:nation_id", patch(patch_nation))
        .with_state(state.clone())
}
