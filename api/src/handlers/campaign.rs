use armies_of_avalon_service::{self, GetAllNationsParams};
use axum::{
    async_trait,
    extract::{FromRequest, FromRequestParts, Path, Query, State},
    http::{request::Parts, HeaderMap, HeaderValue, Request, StatusCode},
    Json,
};
use axum_extra::{
    headers::{
        authorization::{self, Bearer},
        Authorization, Header,
    },
    typed_header::TypedHeaderRejection,
    TypedHeader,
};
use axum_macros::debug_handler;
use entity::{
    campaign_levels::Model as CampaignLevelsModel, nation_armies::Model as NationArmiesModel,
    nations::Model as NationsModel,
};
use serde::Serialize;

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

//#[async_trait]
// impl<T> FromRequestParts<AppState> for TypedHeader<T>
// where
//     T: Header,
// {
//     type Rejection = (StatusCode, &'static str);

//     async fn from_request_parts(
//         parts: &mut Parts,
//         _state: &AppState,
//     ) -> Result<Self, Self::Rejection> {
//         if let Some(authorization) = parts.headers.get("Authorization") {
//             /*
//                 This doesn't work, I get this error:
//                 mismatched types expected type parameter `T`
//                 found reference `&axum::http::HeaderValue`
//             */
//             let val: TypedHeader<HeaderValue> = TypedHeader(authorization.clone());
//             Ok(val)
//         } else {
//             Err((StatusCode::BAD_REQUEST, "`Authorization` header is missing"))
//         }
//     }
// }

#[debug_handler]
pub async fn get_all_campaign_levels(
    //TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<Vec<CampaignLevelsModel>>, (StatusCode, &'static str)> {
    let access_token = headers.get("authorization");
    // Make request with access token to
    // https://dev-10caaad1ieht4kq1.us.auth0.com/userinfo
    println!("AUTHO: {access_token:?}");
    let at_result = armies_of_avalon_service::external_requests::validate_access_token(
        access_token.unwrap().to_str().unwrap().to_string(),
    )
    .await;

    println!("{:?}", at_result);

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
