use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug)]
pub struct AppError(anyhow::Error);

// Tell axum how to convert AppError into a response
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong... {}", self.0),
        )
            .into_response()
    }
}

// enables propagating errors via ?
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
