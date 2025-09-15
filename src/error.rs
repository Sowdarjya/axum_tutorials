use axum::{http::StatusCode, response::{IntoResponse, Response}};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail    
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("{:<12} - {self:?}", "ERROR");

        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
    
}