extern crate aa_battles;

mod handlers;
mod routes;

use armies_of_avalon_service::sea_orm::{Database, DatabaseConnection};
use axum::error_handling::HandleErrorLayer;
use axum::extract::Request;
use axum::http::header::AUTHORIZATION;
use axum::http::{status::InvalidStatusCode, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use axum::{serve, BoxError, Extension, Router};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use migration::{Migrator, MigratorTrait};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use crate::routes::campaign::campaign_routes;
use crate::routes::{
    armies::armies_routes, battles::battles_routes, kingdom::kingdom_routes, users::users_routes,
};
use jsonwebtoken::{
    decode, decode_header, encode,
    errors::ErrorKind,
    jwk::{self, AlgorithmParameters, RSAKeyParameters, RSAKeyType},
    Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
#[derive(Clone, Debug)]
pub struct AppState {
    conn: DatabaseConnection,
}

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let conn = Database::connect(env::var("DATABASE_URL").unwrap().to_owned())
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let state = AppState { conn };
    let app: Router = Router::new()
        .nest("/campaign", campaign_routes(&state))
        .route_layer(axum::middleware::from_fn(authz_check))
        .nest("/battles", battles_routes(&state))
        .nest("/users", users_routes(&state))
        .nest("/armies", armies_routes(&state))
        .nest("/kingdom", kingdom_routes(&state))
        .layer(
            ServiceBuilder::new().layer(CorsLayer::permissive()), // .layer(HandleErrorLayer::new(|_: BoxError| async {
                                                                  //     StatusCode::REQUEST_TIMEOUT
                                                                  // })),
        )
        .with_state(state);

    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");
    println!("{}", server_url);
    let listener = TcpListener::bind(server_url).await.unwrap();

    serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    aud: String,
    sub: String,
    company: String,
    exp: u64,
}
const JWKS_REPLY: &str = r#"
{"keys":[{"kty":"RSA","use":"sig","n":"xG0wuWxCxIhZUKKCzASrqPUZfmamxnvrEeCb8b4v4C--5fY0cDnaLu00JHGpI1jkq2Dl2pF8RCfN0JDlI_q4SivWVNuqHCvHjv8ncdV9pRvuiA6YTTeSNYdxWcZhjTOzqU8WrBDjWWLtZWB6Km8ybI9sIKXrJ-fJ32uFZLxLbz8YkQrxLg3BTerB_JVCqs_MKcHfsGwanWk0PbGruxEoiqhQY1Abzu7PIYPMfsjSYPtSPyWdvGITZ-pJfq9uMoM0nOPRPlYDdzyz8LX940EfsDZQQS8MKTOseHTTdqVzgPSNxERPAtpRa6w7M6TpJ92FD8kSPdP4RgifP95GJ-Uq0Q","e":"AQAB","kid":"RX3veXDIA3lAhUqD2bWGA","x5t":"VasW8p7bGQaRC0S4hLnFC14Zk7c","x5c":["MIIDHTCCAgWgAwIBAgIJTqIDkPHSxVrpMA0GCSqGSIb3DQEBCwUAMCwxKjAoBgNVBAMTIWRldi0xMGNhYWFkMWllaHQ0a3ExLnVzLmF1dGgwLmNvbTAeFw0yMzA5MTMxMzI3NDdaFw0zNzA1MjIxMzI3NDdaMCwxKjAoBgNVBAMTIWRldi0xMGNhYWFkMWllaHQ0a3ExLnVzLmF1dGgwLmNvbTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAMRtMLlsQsSIWVCigswEq6j1GX5mpsZ76xHgm/G+L+AvvuX2NHA52i7tNCRxqSNY5Ktg5dqRfEQnzdCQ5SP6uEor1lTbqhwrx47/J3HVfaUb7ogOmE03kjWHcVnGYY0zs6lPFqwQ41li7WVgeipvMmyPbCCl6yfnyd9rhWS8S28/GJEK8S4NwU3qwfyVQqrPzCnB37BsGp1pND2xq7sRKIqoUGNQG87uzyGDzH7I0mD7Uj8lnbxiE2fqSX6vbjKDNJzj0T5WA3c8s/C1/eNBH7A2UEEvDCkzrHh003alc4D0jcRETwLaUWusOzOk6SfdhQ/JEj3T+EYInz/eRiflKtECAwEAAaNCMEAwDwYDVR0TAQH/BAUwAwEB/zAdBgNVHQ4EFgQUeZTd7X82oci5RdUFwiI4CRcX4I0wDgYDVR0PAQH/BAQDAgKEMA0GCSqGSIb3DQEBCwUAA4IBAQBc/RMj+P3U6mXWd3nFptVy5rVwlaJB9qW94GVOB8Z+ubHdd4+sYsaw6vE+J6N56de/d5++3W6GZ4b4g7vNc1/BQEip/jO3W80qhb8Peygk3ugsuidar9MZpkKW9wwNsXxn/HXw26e2RFj8f1rQhGdNosxlGob6WPfLvzeJyDmyQP2VOcOXaXUFXJSyFb1Zqm+KGrG5k9MJnAyHwFX3z/Tk9jqyUJfbZ7ZhXFj+w4jOr6K5iLVTZpmFwutJRx04wP+ewStjBXhtXtDoL5DzBSU7VOxEjs1ifN90Ze0VdgaX4JqPrRGh9yp1JXvZ9X2H5K3cw+j1zVsfuzXDii7WoKjk"],"alg":"RS256"},{"kty":"RSA","use":"sig","n":"psVjG3KvtiHJWtDJDOIWY_idCD-7sP54iArTpCckRDx5QyRr06hs-9_XV5oSEWrcwpfr7f3LTHSo9NQ7EhjZwVaHhulSCKQ_4I-juKJJzjsGTVmxfSre-hlsQs4RIL07d8ulo6ut8v_qORzibKuAVsnWluHMiQ3rM4124YrZttmztaCG956CGdcRRKnDniLEcjsTOPNP0uR9IWRs6GLR-vq5_EpTk8V9oh9De7Nz9nuuBwhoMAjowaOIy8ab5GcKS_xylT8WwKLjqu_MpNjKgb3WzZyQ4BPj54RWpo7eEILQpvtIOYTMulkLdmMt-b5MONzSDtjpqDUoxO2dIhk6jw","e":"AQAB","kid":"G4Yi2BKyx5fHZmJy4PSh3","x5t":"43kmy-zRdBqFqW5WL5fzBp7cZYo","x5c":["MIIDHTCCAgWgAwIBAgIJZHKCMndbPYo+MA0GCSqGSIb3DQEBCwUAMCwxKjAoBgNVBAMTIWRldi0xMGNhYWFkMWllaHQ0a3ExLnVzLmF1dGgwLmNvbTAeFw0yMzA5MTMxMzI3NDdaFw0zNzA1MjIxMzI3NDdaMCwxKjAoBgNVBAMTIWRldi0xMGNhYWFkMWllaHQ0a3ExLnVzLmF1dGgwLmNvbTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAKbFYxtyr7YhyVrQyQziFmP4nQg/u7D+eIgK06QnJEQ8eUMka9OobPvf11eaEhFq3MKX6+39y0x0qPTUOxIY2cFWh4bpUgikP+CPo7iiSc47Bk1ZsX0q3voZbELOESC9O3fLpaOrrfL/6jkc4myrgFbJ1pbhzIkN6zONduGK2bbZs7WghveeghnXEUSpw54ixHI7EzjzT9LkfSFkbOhi0fr6ufxKU5PFfaIfQ3uzc/Z7rgcIaDAI6MGjiMvGm+RnCkv8cpU/FsCi46rvzKTYyoG91s2ckOAT4+eEVqaO3hCC0Kb7SDmEzLpZC3ZjLfm+TDjc0g7Y6ag1KMTtnSIZOo8CAwEAAaNCMEAwDwYDVR0TAQH/BAUwAwEB/zAdBgNVHQ4EFgQUEw5AtUH1iE1xHJMtGVSdJM91Ld8wDgYDVR0PAQH/BAQDAgKEMA0GCSqGSIb3DQEBCwUAA4IBAQAfVNFM9iw9vl95Ou6aLIUHcZX03ZjRD42g56atPgQUVO3lKJyEE6hDlIeHZAgiMsaqozGi3JlJTtd81zjCwra630ibVrAQSwykaWozIR7zfcO8W+hXRAIaxgoHh9mlj0UFO9n07VjTqMXc5A789gxgZeU9Z524rm5pa6OWoRPq19sFehKKgIhNhrScLHrQ+t1Jdi+s8tlvtrA4aIiEQKDsLliuaKwG9iCww80diJMCbAPgXbBOxf6qxSncgYBeRHqlfo1Qb7z98oCmHK+z+ycJERwu9vQ9vb8k2Ddc3HJffpaOAojG2Vko1/+EwyjXPMaHZcGl2vWbDJm2qpe3+l3E"],"alg":"RS256"}]}
"#;

async fn authz_check(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    println!("TESTESTEST {:?}", req);
    let token_vec_parts_option = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    let token;
    if let Some(token_vec_parts) = token_vec_parts_option {
        token = token_vec_parts.split("Bearer ").collect::<Vec<&str>>()[1];
    } else {
        // Need to figure out how to protect certain routes
        return Ok(next.run(req).await);
    }

    //println!("TOKEN {}", token);
    let jwks: jwk::JwkSet = serde_json::from_str(JWKS_REPLY).unwrap();

    let header = decode_header(&token).unwrap();
    let kid = match header.kid {
        Some(k) => k,
        None => {
            println!("Token doesn't have a `kid` header field");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };
    if let Some(j) = jwks.find(&kid) {
        match &j.algorithm {
            AlgorithmParameters::RSA(rsa) => {
                let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e).unwrap();

                let mut validation = Validation::new(Algorithm::RS256);

                validation.set_audience(&["http://127.0.0.1:8111"]);
                validation.validate_exp = false;

                let decoded_token_result = decode::<HashMap<String, serde_json::Value>>(
                    &token,
                    &decoding_key,
                    &validation,
                );

                if let Ok(decoded_token) = decoded_token_result {
                    println!("DECODED TOKEN:{:?}", decoded_token);
                    let claims = decoded_token.claims;
                    req.extensions_mut().insert(claims);
                    Ok(next.run(req).await)
                } else {
                    println!("Invalid Auth Token");
                    return Err(StatusCode::SERVICE_UNAVAILABLE);
                }
            }
            _ => unreachable!("this should be a RSA"),
        }
    } else {
        println!("No matching JWK found for the given kid");
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }

    //Ok(next.run(req).await)
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
