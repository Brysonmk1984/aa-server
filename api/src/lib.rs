extern crate aa_battles;

mod handlers;
mod middleware;
mod routes;

use armies_of_avalon_service::sea_orm::{Database, DatabaseConnection};
use axum::{serve, BoxError, Extension, Router};

use migration::{Migrator, MigratorTrait};

use std::env;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use crate::middleware::auth::authz_check;
use crate::routes::campaign::campaign_routes;
use crate::routes::{
    armies::armies_routes, battles::battles_routes, kingdom::kingdom_routes, users::users_routes,
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
        .nest("/battles", battles_routes(&state))
        .nest("/kingdom", kingdom_routes(&state))
        //.route_layer(axum::middleware::from_fn(authz_check))
        .nest("/campaign", campaign_routes(&state))
        .nest("/users", users_routes(&state))
        .nest("/armies", armies_routes(&state))
        .layer(
            ServiceBuilder::new().layer(CorsLayer::permissive()),
            // .layer(HandleErrorLayer::new(|_: BoxError| async {
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

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
