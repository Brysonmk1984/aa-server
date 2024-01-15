use std::collections::HashMap;

use crate::AppState;
use armies_of_avalon_service::{Auth0UserPart, Mutation};
use axum::{debug_handler, extract::State, http::StatusCode, Extension, Json};
use entity::users::Model as UsersModel;
use serde_json::Value;

#[debug_handler]
pub async fn create_or_update_user(
    State(state): State<AppState>,
    // Extension(_claims): Extension<HashMap<String, Value>>,
    // warning: uncommenting this causes a silent fail on the FE
    Json(body): Json<Auth0UserPart>,
) -> Result<Json<UsersModel>, (StatusCode, &'static str)> {
    // todo!("Verify that the user from the auth token is the user from the id_token - partial_user.auth0_sub");

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
