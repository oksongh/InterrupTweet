use axum::{http::StatusCode, response::IntoResponse, Json};

#[derive(Debug)]
pub struct FatalError(pub anyhow::Error);

impl<E> From<E> for FatalError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl IntoResponse for FatalError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self.0.to_string())).into_response()
    }
}
