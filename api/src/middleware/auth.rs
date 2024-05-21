use std::{collections::HashMap, env, sync::OnceLock};

use axum::{
    extract::Request,
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{
    decode, decode_header,
    jwk::{self, AlgorithmParameters},
    Algorithm, DecodingKey, Validation,
};
use serde::{Deserialize, Serialize};

fn get_jwks() -> &'static str {
    static JWKS_REPLY: OnceLock<String> = OnceLock::new();
    JWKS_REPLY.get_or_init(|| env::var("JWKS").unwrap())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    aud: String,
    sub: String,
    company: String,
    exp: u64,
}

pub async fn authz_check(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    println!("AuthzCheck {:?}", req);
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

    let jwks: jwk::JwkSet = serde_json::from_str(get_jwks()).unwrap();

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

                // The INTENDED audience, to which the token aud is compared against
                validation.set_audience(&[env::var("TOKEN_AUD").unwrap()]);

                // Was set to false, but should probably be true:
                validation.validate_exp = true;
                println!("VALIDATION: {validation:?}");
                let decoded_token_result = decode::<HashMap<String, serde_json::Value>>(
                    &token,
                    &decoding_key,
                    &validation,
                );

                // @todo Still need to validate permissions on token

                println!("DECODED TOKEN BODY: {decoded_token_result:?}");
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
