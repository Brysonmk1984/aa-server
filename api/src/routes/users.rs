use crate::{handlers::users::create_or_update_user, AppState};
use axum::{routing::post, Router};

pub fn users_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(create_or_update_user))
        .with_state(state.clone())
}
