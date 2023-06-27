use ::entity::armies::Entity as Armies;
use armies_of_avalon_service::sea_orm::{Database, DatabaseConnection};
use armies_of_avalon_service::Query;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router, Server};
use entity::armies::Model;
use migration::{Migrator, MigratorTrait};
use std::str::FromStr;
use std::{env, net::SocketAddr};

#[derive(Clone)]
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

    let app: Router = Router::new().route("/", get(get_armies)).with_state(state);

    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");
    println!("{}", server_url);
    let addr = SocketAddr::from_str(&server_url).unwrap();

    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

async fn get_armies(state: State<AppState>) -> Result<Json<Model>, (StatusCode, &'static str)> {
    let army = Query::find_army_by_id(&state.conn, 1)
        .await
        .expect("Cannot retrieve army by id!")
        .unwrap();

    Ok(Json(army))
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
