use actix_web::dev::ServiceRequest;
use actix_web::http::{HeaderName, HeaderValue};
use actix_web::ResponseError;
use std::fmt;

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

pub struct MiddlewareError(String);

impl MiddlewareError {
    pub fn from_str(s: &str) -> Self {
        MiddlewareError(s.into())
    }
}

impl ResponseError for MiddlewareError {}

impl fmt::Debug for MiddlewareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl fmt::Display for MiddlewareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

// impl Into<Error> for MiddlewareError {
//     fn into(self) -> Error {
//         Error::from(self)
//     }
// }

pub const X_REQUEST_ID: &'static str = "x-request-id";

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
