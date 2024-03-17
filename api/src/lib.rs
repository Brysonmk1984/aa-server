extern crate aa_battles;

mod handlers;
mod middleware;
mod routes;
mod utils;

use aa_battles::types::{Army, ArmyName};
use aa_battles::util::create_hash_of_defaults;
use armies_of_avalon_service::army_service::ArmyQuery;
use armies_of_avalon_service::cron_service::initialize_scheduler;
use armies_of_avalon_service::initialization_service::WeaponArmorQuery;

use axum::{serve, Router};

use migration::sea_orm::{Database, DatabaseConnection};
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
static WEAPON_ARMOR_CELL: OnceLock<HashMap<String, f64>> = OnceLock::new();

/**
 * ARMY_DEFAULT_CELL
 * stores a hash map of f64s for weapon type against armor type
 */
static ARMY_DEFAULT_CELL: OnceLock<HashMap<ArmyName, Army>> = OnceLock::new();

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

    initialize_defaults_to_memory(&state).await.unwrap();
    initialize_scheduler().await?;

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
pub async fn set_weapon_armor_hash(state: &AppState) -> anyhow::Result<()> {
    let weapon_armor_result: Vec<entity::weapon_armor::Model> =
        WeaponArmorQuery::get_weapon_armor_reduction_values(&state.conn).await?;

    let mut weapon_armor_hashmap: HashMap<String, f64> = HashMap::new();

    weapon_armor_result.into_iter().for_each(|item| {
        let key = item.weapon + "-" + item.armor.unwrap().as_str();
        weapon_armor_hashmap.insert(key, item.reduction.try_into().unwrap());
    });

    let _ = WEAPON_ARMOR_CELL.set(weapon_armor_hashmap);

    Ok(())
}

pub async fn initialize_defaults_to_memory(state: &AppState) -> anyhow::Result<()> {
    set_weapon_armor_hash(state).await?;

    let result = ArmyQuery::get_all_armies(&state.conn).await?;
    let mut army_defaults: Vec<Army> = result
        .iter()
        .map(|army| army.clone().into())
        .collect::<Vec<Army>>();

    army_defaults.sort_by(|a, b| a.id.cmp(&b.id));

    let army_default_hash = create_hash_of_defaults(army_defaults);

    let _ = ARMY_DEFAULT_CELL.set(army_default_hash);
    Ok(())
}
