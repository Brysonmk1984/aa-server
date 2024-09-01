extern crate aa_battles;

mod handlers;
mod middleware;
mod routes;
mod utils;

use aa_battles::types::Army;
use aa_battles::types::ArmyName;
use aa_battles::util::create_hash_of_defaults;
use armies_of_avalon_service::army_service::ArmyQuery;
use armies_of_avalon_service::cron_service::initialize_scheduler;
use armies_of_avalon_service::initialization_service::AoeSpreadQuery;
use armies_of_avalon_service::initialization_service::WeaponArmorQuery;

use axum::{serve, Router};

use migration::sea_orm::{Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};
use routes::game::game_routes;
use serde::Serialize;

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
 * AOE_SPREAD_CELL
 * stores a hash map of f64s for aoe impact against different spread values
 */
static AOE_SPREAD_CELL: OnceLock<HashMap<i32, Vec<(f64, i32)>>> = OnceLock::new();

/**
 * CAMPAIGN_LEVEL_REWARDS
 * stores a hashmap of CampaignLevelRewards
 */

static CAMPAIGN_LEVEL_REWARDS_CELL: OnceLock<HashMap<i32, (i32, Reward)>> = OnceLock::new();

/**
 * ARMY_DEFAULT_CELL
 * stores a hash map of f64s for weapon type against armor type
 */
static ARMY_DEFAULT_CELL: OnceLock<HashMap<ArmyName, Army>> = OnceLock::new();

#[derive(Clone, Debug)]
pub struct AppState {
    conn: DatabaseConnection,
    income_calc_minutes: String,
    upkeep_calc_minutes: String,
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

    let state = AppState {
        conn,
        income_calc_minutes: env::var("INCOME_CALCULATIONS_EVERY_X_MINUTES").unwrap(),
        upkeep_calc_minutes: env::var("UPKEEP_CALCULATIONS_EVERY_X_MINUTES").unwrap(),
    };

    initialize_defaults_to_memory(&state).await.unwrap();

    let enable_scheduler = env::var("ENABLE_SCHEDULER")
        .unwrap()
        .parse::<bool>()
        .unwrap();

    if enable_scheduler {
        initialize_scheduler(
            &state.conn,
            &state.income_calc_minutes.as_str(),
            &state.upkeep_calc_minutes.as_str(),
        )
        .await?;
    }

    let app: Router = Router::new()
        .nest("/battles", battles_routes(&state))
        .nest("/kingdom", kingdom_routes(&state))
        .route_layer(axum::middleware::from_fn(authz_check))
        .nest("/campaign", campaign_routes(&state))
        .nest("/users", users_routes(&state))
        .nest("/armies", armies_routes(&state))
        .nest("/game", game_routes(&state))
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
async fn set_weapon_armor_hash(state: &AppState) -> anyhow::Result<()> {
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

/**
 * fn set_aoe_spread_hash
 * used for initializing the aoe_spread calculations
 */
async fn set_aoe_spread_hash(state: &AppState) -> anyhow::Result<()> {
    let aoe_spread_result: Vec<entity::aoe_spread::Model> =
        AoeSpreadQuery::get_aoe_spread_values(&state.conn).await?;

    let update_hash_map: HashMap<i32, Vec<(f64, i32)>> = aoe_spread_result.iter().fold(
        HashMap::from([(1, Vec::new()), (2, Vec::new()), (3, Vec::new())]),
        |mut acc, cur| {
            let aoe: f64 = cur.aoe.try_into().unwrap();
            let val = acc.get_mut(&cur.spread).unwrap();
            val.push((aoe, cur.hits));
            acc
        },
    );
    println!("{update_hash_map:?}");
    let _ = AOE_SPREAD_CELL.set(update_hash_map);
    Ok(())
}

#[derive(Clone, Copy, Debug, Serialize)]
pub enum Reward {
    Gold,
    Enlist(ArmyName),
}

async fn set_campaign_level_rewards_hash() -> anyhow::Result<()> {
    let map: HashMap<i32, (i32, Reward)> = HashMap::from([
        (1, (100, Reward::Enlist(ArmyName::MinuteMenMilitia))),
        (2, (100, Reward::Enlist(ArmyName::PeacekeeperMonks))),
        (3, (1000, Reward::Gold)),
        (4, (100, Reward::Enlist(ArmyName::AmazonianHuntresses))),
        (5, (1200, Reward::Gold)),
        (6, (100, Reward::Enlist(ArmyName::NorthWatchLongbowmen))),
        (7, (1300, Reward::Gold)),
        (8, (100, Reward::Enlist(ArmyName::RoninImmortals))),
        (
            9,
            (100, Reward::Enlist(ArmyName::BarbariansOfTheOuterSteppe)),
        ),
        (10, (1500, Reward::Gold)),
        (11, (100, Reward::Enlist(ArmyName::DeathDealerAssassins))),
        (12, (1700, Reward::Gold)),
        (13, (100, Reward::Enlist(ArmyName::OathSwornKnights))),
        (14, (1900, Reward::Gold)),
        (15, (2000, Reward::Gold)),
        (16, (100, Reward::Enlist(ArmyName::CastlegateCrossbowmen))),
        (17, (2100, Reward::Gold)),
        (18, (2200, Reward::Gold)),
        (19, (100, Reward::Enlist(ArmyName::MagiEnforcers))),
        (20, (2300, Reward::Gold)),
        (21, (100, Reward::Enlist(ArmyName::ShinobiMartialArtists))),
        (22, (2500, Reward::Gold)),
        (23, (2600, Reward::Gold)),
        (24, (2700, Reward::Gold)),
        (25, (100, Reward::Enlist(ArmyName::AvianCliffDwellers))),
        (26, (100, Reward::Enlist(ArmyName::HighbornCavalry))),
        (27, (2900, Reward::Gold)),
        (28, (3000, Reward::Gold)),
        (29, (100, Reward::Enlist(ArmyName::SkullClanDeathCultists))),
    ]);

    let _ = CAMPAIGN_LEVEL_REWARDS_CELL.set(map);

    Ok(())
}

pub async fn initialize_defaults_to_memory(state: &AppState) -> anyhow::Result<()> {
    set_weapon_armor_hash(state).await?;
    set_aoe_spread_hash(state).await?;
    set_campaign_level_rewards_hash().await?;

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
