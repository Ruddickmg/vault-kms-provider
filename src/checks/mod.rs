use bytes::Bytes;
use http::{Request, Response, StatusCode};
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::io::Error;
use tokio::net::TcpListener;
use tracing::{error, info, instrument};

extern crate lib;

mod health;
mod readiness;

async fn checks(
    request: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
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

#[instrument]
pub async fn serve() -> Result<(), Error> {
    let http_address = lib::configuration::health::HealthCheckConfiguration::new();
    let addr = SocketAddr::from_str(&http_address.endpoint).expect(&format!(
        "Invalid http address: {:?}",
        http_address.endpoint
    ));
    let listener = TcpListener::bind(addr).await?;
    info!(
        "Health and liveness checks listening at: {:?}",
        addr.to_string()
    );
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(checks))
                .await
            {
                error!("Error serving connection: {:?}", err);
            }
        });
    }
}
