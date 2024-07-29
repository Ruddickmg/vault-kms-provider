use std::convert::Infallible;
use bytes::Bytes;
use http::{Request, Response, StatusCode};
use http_body_util::Full;

pub async fn health_check(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
  Ok(Response::builder()
    .status(StatusCode::OK)
    .body(Full::new(Bytes::from("OK")))
    .expect("Unable to build request"))
}
