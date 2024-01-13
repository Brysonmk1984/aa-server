use std::collections::HashMap;

use armies_of_avalon_service::{self, GetAllNationsParams};
use axum::{
    debug_handler,
    extract::{Path, Query, State},
    http::{status::InvalidStatusCode, StatusCode},
    Json,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use entity::{
    campaign_levels::Model as CampaignLevelsModel, nation_armies::Model as NationArmiesModel,
    nations::Model as NationsModel,
};
use jsonwebtoken::{
    decode, decode_header, encode,
    errors::ErrorKind,
    jwk::{self, AlgorithmParameters, RSAKeyParameters, RSAKeyType},
    Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[debug_handler]
pub async fn get_all_campaign_nations_details(
    State(state): State<AppState>,
    Query(params): Query<GetAllNationsParams>,
) -> Result<Json<Vec<NationsModel>>, (StatusCode, &'static str)> {
    let nations: Vec<NationsModel> = armies_of_avalon_service::Query::get_all_nations(
        &state.conn,
        params,
    )
    .await
    .expect(
        "A vec of nations  should return when fetching with or without the is_npc query param!",
    );

    return Ok(Json(nations));
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    sub: String,
    company: String,
    exp: u64,
}
const JWKS_REPLY: &str = r#"
{"keys":[{"kty":"RSA","use":"sig","n":"xG0wuWxCxIhZUKKCzASrqPUZfmamxnvrEeCb8b4v4C--5fY0cDnaLu00JHGpI1jkq2Dl2pF8RCfN0JDlI_q4SivWVNuqHCvHjv8ncdV9pRvuiA6YTTeSNYdxWcZhjTOzqU8WrBDjWWLtZWB6Km8ybI9sIKXrJ-fJ32uFZLxLbz8YkQrxLg3BTerB_JVCqs_MKcHfsGwanWk0PbGruxEoiqhQY1Abzu7PIYPMfsjSYPtSPyWdvGITZ-pJfq9uMoM0nOPRPlYDdzyz8LX940EfsDZQQS8MKTOseHTTdqVzgPSNxERPAtpRa6w7M6TpJ92FD8kSPdP4RgifP95GJ-Uq0Q","e":"AQAB","kid":"RX3veXDIA3lAhUqD2bWGA","x5t":"VasW8p7bGQaRC0S4hLnFC14Zk7c","x5c":["MIIDHTCCAgWgAwIBAgIJTqIDkPHSxVrpMA0GCSqGSIb3DQEBCwUAMCwxKjAoBgNVBAMTIWRldi0xMGNhYWFkMWllaHQ0a3ExLnVzLmF1dGgwLmNvbTAeFw0yMzA5MTMxMzI3NDdaFw0zNzA1MjIxMzI3NDdaMCwxKjAoBgNVBAMTIWRldi0xMGNhYWFkMWllaHQ0a3ExLnVzLmF1dGgwLmNvbTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAMRtMLlsQsSIWVCigswEq6j1GX5mpsZ76xHgm/G+L+AvvuX2NHA52i7tNCRxqSNY5Ktg5dqRfEQnzdCQ5SP6uEor1lTbqhwrx47/J3HVfaUb7ogOmE03kjWHcVnGYY0zs6lPFqwQ41li7WVgeipvMmyPbCCl6yfnyd9rhWS8S28/GJEK8S4NwU3qwfyVQqrPzCnB37BsGp1pND2xq7sRKIqoUGNQG87uzyGDzH7I0mD7Uj8lnbxiE2fqSX6vbjKDNJzj0T5WA3c8s/C1/eNBH7A2UEEvDCkzrHh003alc4D0jcRETwLaUWusOzOk6SfdhQ/JEj3T+EYInz/eRiflKtECAwEAAaNCMEAwDwYDVR0TAQH/BAUwAwEB/zAdBgNVHQ4EFgQUeZTd7X82oci5RdUFwiI4CRcX4I0wDgYDVR0PAQH/BAQDAgKEMA0GCSqGSIb3DQEBCwUAA4IBAQBc/RMj+P3U6mXWd3nFptVy5rVwlaJB9qW94GVOB8Z+ubHdd4+sYsaw6vE+J6N56de/d5++3W6GZ4b4g7vNc1/BQEip/jO3W80qhb8Peygk3ugsuidar9MZpkKW9wwNsXxn/HXw26e2RFj8f1rQhGdNosxlGob6WPfLvzeJyDmyQP2VOcOXaXUFXJSyFb1Zqm+KGrG5k9MJnAyHwFX3z/Tk9jqyUJfbZ7ZhXFj+w4jOr6K5iLVTZpmFwutJRx04wP+ewStjBXhtXtDoL5DzBSU7VOxEjs1ifN90Ze0VdgaX4JqPrRGh9yp1JXvZ9X2H5K3cw+j1zVsfuzXDii7WoKjk"],"alg":"RS256"},{"kty":"RSA","use":"sig","n":"psVjG3KvtiHJWtDJDOIWY_idCD-7sP54iArTpCckRDx5QyRr06hs-9_XV5oSEWrcwpfr7f3LTHSo9NQ7EhjZwVaHhulSCKQ_4I-juKJJzjsGTVmxfSre-hlsQs4RIL07d8ulo6ut8v_qORzibKuAVsnWluHMiQ3rM4124YrZttmztaCG956CGdcRRKnDniLEcjsTOPNP0uR9IWRs6GLR-vq5_EpTk8V9oh9De7Nz9nuuBwhoMAjowaOIy8ab5GcKS_xylT8WwKLjqu_MpNjKgb3WzZyQ4BPj54RWpo7eEILQpvtIOYTMulkLdmMt-b5MONzSDtjpqDUoxO2dIhk6jw","e":"AQAB","kid":"G4Yi2BKyx5fHZmJy4PSh3","x5t":"43kmy-zRdBqFqW5WL5fzBp7cZYo","x5c":["MIIDHTCCAgWgAwIBAgIJZHKCMndbPYo+MA0GCSqGSIb3DQEBCwUAMCwxKjAoBgNVBAMTIWRldi0xMGNhYWFkMWllaHQ0a3ExLnVzLmF1dGgwLmNvbTAeFw0yMzA5MTMxMzI3NDdaFw0zNzA1MjIxMzI3NDdaMCwxKjAoBgNVBAMTIWRldi0xMGNhYWFkMWllaHQ0a3ExLnVzLmF1dGgwLmNvbTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAKbFYxtyr7YhyVrQyQziFmP4nQg/u7D+eIgK06QnJEQ8eUMka9OobPvf11eaEhFq3MKX6+39y0x0qPTUOxIY2cFWh4bpUgikP+CPo7iiSc47Bk1ZsX0q3voZbELOESC9O3fLpaOrrfL/6jkc4myrgFbJ1pbhzIkN6zONduGK2bbZs7WghveeghnXEUSpw54ixHI7EzjzT9LkfSFkbOhi0fr6ufxKU5PFfaIfQ3uzc/Z7rgcIaDAI6MGjiMvGm+RnCkv8cpU/FsCi46rvzKTYyoG91s2ckOAT4+eEVqaO3hCC0Kb7SDmEzLpZC3ZjLfm+TDjc0g7Y6ag1KMTtnSIZOo8CAwEAAaNCMEAwDwYDVR0TAQH/BAUwAwEB/zAdBgNVHQ4EFgQUEw5AtUH1iE1xHJMtGVSdJM91Ld8wDgYDVR0PAQH/BAQDAgKEMA0GCSqGSIb3DQEBCwUAA4IBAQAfVNFM9iw9vl95Ou6aLIUHcZX03ZjRD42g56atPgQUVO3lKJyEE6hDlIeHZAgiMsaqozGi3JlJTtd81zjCwra630ibVrAQSwykaWozIR7zfcO8W+hXRAIaxgoHh9mlj0UFO9n07VjTqMXc5A789gxgZeU9Z524rm5pa6OWoRPq19sFehKKgIhNhrScLHrQ+t1Jdi+s8tlvtrA4aIiEQKDsLliuaKwG9iCww80diJMCbAPgXbBOxf6qxSncgYBeRHqlfo1Qb7z98oCmHK+z+ycJERwu9vQ9vb8k2Ddc3HJffpaOAojG2Vko1/+EwyjXPMaHZcGl2vWbDJm2qpe3+l3E"],"alg":"RS256"}]}
"#;

#[debug_handler]
pub async fn get_all_campaign_levels(
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    State(state): State<AppState>,
) -> Result<Json<Vec<CampaignLevelsModel>>, (StatusCode, &'static str)> {
    let token: String = authorization.0.token().to_string();

    let jwks: jwk::JwkSet = serde_json::from_str(JWKS_REPLY).unwrap();

    let header = decode_header(&token).unwrap();
    let kid = match header.kid {
        Some(k) => k,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Token doesn't have a `kid` header field",
            ))
        }
    };
    if let Some(j) = jwks.find(&kid) {
        match &j.algorithm {
            AlgorithmParameters::RSA(rsa) => {
                let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e).unwrap();

                let mut validation = Validation::new(Algorithm::RS256);

                validation.set_audience(&["http://127.0.0.1:8111"]);
                validation.validate_exp = false;

                let decoded_token = decode::<HashMap<String, serde_json::Value>>(
                    &token,
                    &decoding_key,
                    &validation,
                )
                .unwrap();
                println!("DECODED TOKEN:{:?}", decoded_token);
            }
            _ => unreachable!("this should be a RSA"),
        }
    } else {
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            "No matching JWK found for the given kid",
        ));
    }

    let mut campaign_levels: Vec<CampaignLevelsModel> =
        armies_of_avalon_service::Query::get_all_campaign_levels(&state.conn)
            .await
            .expect("A vec of campaign levels should be returned");
    campaign_levels.sort_by_key(|campaign_level| campaign_level.level);
    return Ok(Json(campaign_levels));
}

#[derive(Serialize)]
pub struct NationWithArmies {
    nation_details: NationsModel,
    all_armies: Vec<NationArmiesModel>,
}

#[debug_handler]
pub async fn get_campaign_nation_details(
    State(state): State<AppState>,
    Path(nation_id): Path<i32>,
) -> Result<Json<NationWithArmies>, (StatusCode, &'static str)> {
    println!("test {nation_id} asd");
    let (nation_details, all_armies) =
        armies_of_avalon_service::Query::get_nation_with_nation_armies_by_nation_id(
            &state.conn,
            nation_id,
        )
        .await
        .expect("A Nation and a vec of nation armies should return when fetching by nation id!");

    let combined_nation_armies = NationWithArmies {
        nation_details,
        all_armies,
    };
    Ok(Json(combined_nation_armies))
}
