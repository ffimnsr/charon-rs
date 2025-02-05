use axum::http::StatusCode;
use axum::response::{
  IntoResponse,
  Response,
};
use std::result::Result as StdResult;

pub type Result<T, E = AppError> = StdResult<T, E>;

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    (
      StatusCode::INTERNAL_SERVER_ERROR,
      format!("Something went wrong: {}", self.0),
    )
      .into_response()
  }
}

impl<E> From<E> for AppError
where
  E: Into<anyhow::Error>,
{
  fn from(value: E) -> Self {
    Self(value.into())
  }
}
