use actix_web::body::AnyBody;
use actix_web::dev::ServiceRequest;
use actix_web::http::header::CONTENT_TYPE;
use actix_web::http::{header, HeaderName, HeaderValue, StatusCode};
use actix_web::web::BytesMut;
use actix_web::{HttpResponse, ResponseError};
use std::fmt;
use std::fmt::Write;

#[macro_export]
macro_rules! middleware {
    ($req:tt,$b:tt) => {
        $b($req)
    };
    ($req:tt,$a:tt,$($b:tt),*) => {
        match $a($req) {
            Ok(_) => {
                middleware!($req,$($b),*)
            }
            Err(e) => {
                Err(e)
            }
        }
    }
}

pub struct MiddlewareError {
    code: StatusCode,
    message: String,
}

impl MiddlewareError {
    pub fn from(code: StatusCode, s: &str) -> Self {
        MiddlewareError {
            code,
            message: s.into(),
        }
    }
}

impl ResponseError for MiddlewareError {
    fn status_code(&self) -> StatusCode {
        self.code
    }
    fn error_response(&self) -> HttpResponse {
        let mut res = HttpResponse::new(self.status_code());
        let mut buf = BytesMut::new();
        let _ = write!(&mut buf, "{}", self);
        res.headers_mut()
            .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        res.set_body(AnyBody::from(buf))
    }
}

impl fmt::Debug for MiddlewareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl fmt::Display for MiddlewareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!(
            "{{\"code\":{},\"message\":{}}}",
            self.code.as_u16(),
            self.message
        );
        f.write_str(s.as_str())
    }
}

pub const X_REQUEST_ID: &'static str = "x-request-id";

/// Server middlewares
///
/// Err(MiddlewareError::from(StatusCode::INTERNAL_SERVER_ERROR, "hello error"))
pub fn with_print(req: &mut ServiceRequest) -> Result<(), MiddlewareError> {
    debug!("request incoming:{:?}", req);
    Ok(())
}

pub fn with_trace(req: &mut ServiceRequest) -> Result<(), MiddlewareError> {
    let trace_id = uuid::Uuid::new_v4().to_string();
    req.headers_mut().insert(
        HeaderName::from_static(X_REQUEST_ID),
        HeaderValue::from_bytes(trace_id.as_bytes()).unwrap(),
    );
    Ok(())
}
