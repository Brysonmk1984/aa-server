use axum::{routing::get, Router};

use crate::{
    handlers::campaign::{get_all_campaign_levels, get_campaign_nation_details},
    AppState,
};

pub fn campaign_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/levels", get(get_all_campaign_levels))
        .route("/levels/:level/nation", get(get_campaign_nation_details))
        .with_state(state.clone())
}
