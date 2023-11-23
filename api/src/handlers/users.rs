use crate::AppState;
use armies_of_avalon_service::{Auth0UserPart, Mutation};
use axum::{extract::State, http::StatusCode, Json};
use axum_macros::debug_handler;
use entity::users::Model as UsersModel;

#[debug_handler]
pub async fn create_or_update_user(
    State(state): State<AppState>,
    Json(body): Json<Auth0UserPart>,
) -> Result<Json<UsersModel>, (StatusCode, &'static str)> {
    let partial_user = Auth0UserPart {
        email: body.email.to_string(),
        email_verified: body.email_verified,
        auth0_sub: body.auth0_sub.to_string(),
    };

    let user = Mutation::insert_or_return_user(&state.conn, partial_user)
        .await
        .expect("Could not insert or return user!");

    Ok(Json(user))
}
