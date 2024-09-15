use axum::{debug_handler, Json};

use crate::utils::error::AppError;

#[debug_handler]
pub async fn run_test_battle() -> Result<Json<()>, AppError> {
    Ok(Json(()))
}
