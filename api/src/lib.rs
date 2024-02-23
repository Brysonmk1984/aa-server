extern crate aa_battles;

mod handlers;
mod middleware;
mod routes;
mod utils;

use armies_of_avalon_service::sea_orm::{Database, DatabaseConnection};
use axum::{serve, Router};

use migration::{Migrator, MigratorTrait};

use std::collections::HashMap;
use std::env;
use std::sync::OnceLock;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use crate::middleware::auth::authz_check;
use crate::routes::campaign::campaign_routes;
use crate::routes::{
    armies::armies_routes, battles::battles_routes, kingdom::kingdom_routes, users::users_routes,
};

/**
 * WEAPON_ARMOR_CELL
 * stores a hash map of f64s for weapon type against armor type
 */
static WEAPON_ARMOR_CELL: OnceLock<HashMap<&'static str, f64>> = OnceLock::new();

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

    set_weapon_armor_hash();

    let state = AppState { conn };
    let app: Router = Router::new()
        .nest("/battles", battles_routes(&state))
        .nest("/kingdom", kingdom_routes(&state))
        .route_layer(axum::middleware::from_fn(authz_check))
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

/**
 * fn set_weapon_armor_map
 * used for initializing the chance to hit given weapon type against armor type
 */
pub fn set_weapon_armor_hash() {
    let map = HashMap::from([
        ("piercing-unarmored", 1.0),
        ("piercing-leather", 0.75),
        ("piercing-chain", 0.6),
        ("piercing-plate", 0.1),
        ("crushing-unarmored", 0.25),
        ("crushing-leather", 0.50),
        ("crushing-chain", 0.75),
        ("crushing-plate", 1.0),
        ("blunt-unarmored", 0.75),
        ("blunt-leather", 0.75),
        ("blunt-chain", 0.5),
        ("blunt-plate", 0.25),
        ("edged-unarmored", 1.0),
        ("edged-leather", 0.75),
        ("edged-chain", 0.5),
        ("edged-plate", 0.25),
        ("magic-unarmored", 0.25),
        ("magic-leather", 0.50),
        ("magic-chain", 1.0),
        ("magic-plate", 0.75),
    ]);
    WEAPON_ARMOR_CELL.set(map);
}
