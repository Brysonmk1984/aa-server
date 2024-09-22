use std::env;

use reqwest::{header::AUTHORIZATION, Error};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AccessToken {
    // sub: String,
    // nickname: String,
    // name: String,
    // picture: String,
    // updated_at: String,
    // email: String,
    // email_verified: bool,
}

/**
 * An alternative way to validate access tokens. Not currently in use since we're validating on our server
 */
pub async fn validate_access_token(token: String) -> Result<AccessToken, Error> {
    let endpoint = env::var("TOKEN_VALIDATION_ENDPOINT").unwrap().to_owned();
    let client = reqwest::Client::new();
    let body = client
        .get(endpoint)
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .send()
        .await?
        .json::<AccessToken>()
        .await?;

    Ok(body)
}
