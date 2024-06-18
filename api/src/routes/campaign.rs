use axum::{routing::get, Router};

use crate::{
    handlers::campaign::{
        get_all_campaign_levels, get_campaign_nation_details, get_highest_campaign_level_completed,
    },
    AppState,
};

pub fn campaign_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/levels", get(get_all_campaign_levels))
        .route("/levels/:level/nation", get(get_campaign_nation_details))
        .route(
            "/nation/:nation_id/highest_completed",
            get(get_highest_campaign_level_completed),
        )
        .with_state(state.clone())
}
