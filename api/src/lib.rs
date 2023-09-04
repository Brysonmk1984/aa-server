use armies_of_avalon_service::sea_orm::{Database, DatabaseConnection};
use armies_of_avalon_service::Query;
use axum::http::Method;
use axum::{extract::State, http::StatusCode, routing::get, Json, Router, Server};
use axum_macros::debug_handler;
use entity::armies::Model;
// use entity::nation_armies::Model as NationArmiesModel;
// use entity::nations::Model as NationsModel;
use migration::{Migrator, MigratorTrait};
use std::str::FromStr;
use std::{env, net::SocketAddr};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

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
        //.route("/:id", get(get_army))
        .layer(
            ServiceBuilder::new().layer(
                CorsLayer::new()
                    // allow `GET` and `POST` when accessing the resource
                    .allow_methods([
                        Method::GET,
                        Method::POST,
                        Method::PATCH,
                        Method::PUT,
                        Method::DELETE,
                    ])
                    // allow requests from any origin
                    .allow_origin(Any),
            ),
        )
        .with_state(state);

    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");
    println!("{}", server_url);
    let addr = SocketAddr::from_str(&server_url).unwrap();

    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
async fn get_all_armies(
    state: State<AppState>,
) -> Result<Json<Vec<Model>>, (StatusCode, &'static str)> {
    let armies = Query::get_all_armies(&state.conn)
        .await
        .expect("Cannot get all armies!");

    Ok(Json(armies))
}

// async fn get_army(
//     state: State<AppState>,
//     Path(id): Path<i32>,
// ) -> Result<Json<ArmyModel>, (StatusCode, &'static str)> {
//     let army = Query::find_army_by_id(&state.conn, id)
//         .await
//         .expect("Cannot retrieve army by id!")
//         .unwrap();

//     Ok(Json(army))
// }

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
