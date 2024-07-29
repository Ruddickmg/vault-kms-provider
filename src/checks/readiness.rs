use bytes::Bytes;
use http::{Request, Response, StatusCode};
use http_body_util::Full;
use std::convert::Infallible;
use std::path::Path;

extern crate lib;

pub async fn readiness_check(
    _: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let socket_path = lib::configuration::socket().socket_path;
    let status = if Path::new(&socket_path).exists() {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    };
    Ok(Response::builder()
        .status(status)
        .body(Full::new(Bytes::from("Not ready")))
        .expect("Unable to build request"))
}
