use derive_more::{Display, Error};
use ntex::{http, web};

#[derive(Debug, Display, Error)]
pub enum HttpError {
    #[display("internal error")]
    InternalError,

    #[display("bad request")]
    BadClientData,

    #[display("timeout")]
    Timeout,

    #[display("Unprocessable Entity")]
    UnprocessableEntity,

    #[display("Not Found")]
    NotFound,
}

impl web::error::WebResponseError for HttpError {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        web::HttpResponse::build(self.status_code())
            .set_header("content-type", "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> http::StatusCode {
        match *self {
            HttpError::InternalError => http::StatusCode::INTERNAL_SERVER_ERROR,
            HttpError::BadClientData => http::StatusCode::BAD_REQUEST,
            HttpError::Timeout => http::StatusCode::GATEWAY_TIMEOUT,
            HttpError::UnprocessableEntity => http::StatusCode::UNPROCESSABLE_ENTITY,
            HttpError::NotFound => http::StatusCode::NOT_FOUND,
        }
    }
}
