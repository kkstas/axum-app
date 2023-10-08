use axum::http::StatusCode;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    LoginFail,
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    TicetDeleteFailIdNotFound { id: u64 },
    AuthFailCtxNotInRequestExt,
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
