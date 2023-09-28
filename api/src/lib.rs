#![allow(warnings)]
use armies_of_avalon_service::sea_orm::{Database, DatabaseConnection};
use armies_of_avalon_service::{Auth0UserPart, Mutation, Query};
use axum::body::{self, Body};
use axum::extract::Path;
use axum::http::{HeaderName, Method, Request};
use axum::routing::{post, put};
use axum::{extract::State, http::StatusCode, routing::get, Json, Router, Server};

use axum_macros::debug_handler;
use entity::armies::Model as ArmiesModel;
use entity::nation_armies::Model as NationArmiesModel;
use entity::nations::Model as NationsModel;
use entity::users::Model as UsersModel;
use migration::{Migrator, MigratorTrait};
use std::str::FromStr;
use std::{env, net::SocketAddr};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

//pub const ACCESS_CONTROL_ALLOW_METHODS: HeaderName::from_static("Content-Type");

#[derive(Clone, Debug)]
struct AppState {
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
        .route("/", get(get_all_armies))
        .route("/nation-profile/:user_id", get(get_nation_and_armies))
        .route("/users", post(create_or_update_user))
        .route("/matchup", get(get_matchup))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");
    println!("{}", server_url);
    let addr = SocketAddr::from_str(&server_url).unwrap();

    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
#[debug_handler]
async fn get_all_armies(
    state: State<AppState>,
) -> Result<Json<Vec<ArmiesModel>>, (StatusCode, &'static str)> {
    let armies = Query::get_all_armies(&state.conn)
        .await
        .expect("Cannot get all armies!");

    Ok(Json(armies))
}

#[debug_handler]
async fn get_matchup(
    state: State<AppState>,
) -> Result<
    Json<Vec<(entity::nations::Model, Vec<entity::nation_armies::Model>)>>,
    (StatusCode, &'static str),
> {
    let mut nation_and_nation_armies_one = Query::get_nation_with_nation_armies(&state.conn, 1)
        .await
        .expect("Cannot get nation with armies!");
    let mut nation_and_nation_armies_two = Query::get_nation_with_nation_armies(&state.conn, 2)
        .await
        .expect("Cannot get nation with armies!");

    println!("{nation_and_nation_armies_one:?} {nation_and_nation_armies_two:?}");
    nation_and_nation_armies_one.append(&mut nation_and_nation_armies_two);

    Ok(Json(nation_and_nation_armies_one))
}

#[debug_handler]
async fn create_or_update_user(
    State(state): State<AppState>,
    Json(body): Json<Auth0UserPart>,
) -> Result<Json<UsersModel>, (StatusCode, &'static str)> {
    println!("{body:?}");
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

#[debug_handler]
async fn get_nation_and_armies(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<Json<(NationsModel, Vec<NationArmiesModel>)>, (StatusCode, &'static str)> {
    println!("'LOOOOK, {}", &user_id);
    let nation_and_armies = Query::get_nation_with_nation_armies_by_user_id(&state.conn, user_id)
        .await
        .expect("A Nation and a vec of nation armies should return!");
    dbg!(&nation_and_armies);

    Ok(Json(nation_and_armies))
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
