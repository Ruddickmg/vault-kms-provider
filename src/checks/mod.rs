use std::convert::Infallible;
use std::net::SocketAddr;
use bytes::Bytes;
use http::{Request, Response, StatusCode};
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use tokio::io::Error;

mod health;
mod readiness;

async fn checks(request: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
  let path = request.uri().path();
  if path.contains("ready") {
    readiness::readiness_check(request).await
  } else if path.contains("health") {
    health::health_check(request).await
  } else {
    Ok(Response::builder()
      .status(StatusCode::NOT_FOUND)
      .body(Full::new(Bytes::from("Not found")))
      .expect("Unable to build response"))
  }
}

pub async fn serve() -> Result<(), Error> {
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  let listener = TcpListener::bind(addr).await?;
  println!("Health and liveness checks listening at: {:?}", addr.to_string());
  loop {
    let (stream, _) = listener.accept().await?;
    let io = TokioIo::new(stream);

    tokio::task::spawn(async move {
      if let Err(err) = http1::Builder::new()
        .serve_connection(io, service_fn(checks))
        .await
      {
        eprintln!("Error serving connection: {:?}", err);
      }
    });
  }
}