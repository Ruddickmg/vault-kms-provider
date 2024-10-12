use crate::configuration::socket::SocketConfiguration;
use bytes::Bytes;
use http::{Response, StatusCode};
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::net::TcpListener;
use tracing::{error, info, instrument};

mod health;
mod readiness;

async fn checks(uri: String, socket_path: String) -> Result<Response<Full<Bytes>>, Infallible> {
    if uri.contains("ready") {
        readiness::readiness_check(&socket_path).await
    } else if uri.contains("health") {
        health::health_check().await
    } else {
        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::new(Bytes::from("Not found")))
            .expect("Unable to build response"))
    }
}

#[instrument]
pub async fn serve(http_address: &str) -> Result<(), std::io::Error> {
    let addr = SocketAddr::from_str(&http_address)
        .expect(&format!("Invalid http address: {:?}", http_address));
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
                .serve_connection(
                    io,
                    service_fn(|request| {
                        checks(
                            request.uri().path().to_string(),
                            SocketConfiguration::silent().socket_path,
                        )
                    }),
                )
                .await
            {
                error!("Error serving connection: {:?}", err);
            }
        });
    }
}

#[cfg(test)]
mod serve {
    use super::serve;
    use reqwest::StatusCode;
    use std::time::Duration;

    #[tokio::test]
    async fn responds_to_http_requests() {
        let path = "127.0.0.1:8085";
        let result = tokio::select! {
            r = async {
                tokio::time::sleep(Duration::from_millis(100)).await;
                reqwest::get(&format!("http://{}/health", path)).await.unwrap().status()
            } => Some(r),
            _ = async {
                serve(path).await.unwrap()
            } => None
        };
        assert_eq!(result, Some(StatusCode::OK));
    }
}

#[cfg(test)]
mod checks {
    use super::checks;
    use http;
    use http::StatusCode;

    #[tokio::test]
    async fn ready_returns_ok_if_socket_exists() {
        let resp = checks(
            "/ready".to_string(),
            "test_files/vault-kms-provider.yaml".to_string(),
        )
        .await
        .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn ready_returns_error_status_if_socket_does_not_exist() {
        let resp = checks(
            "/ready".to_string(),
            "test_files/non-existent-file".to_string(),
        )
        .await
        .unwrap();
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn health_returns_ok_if_app_is_working() {
        let resp = checks(
            "/health".to_string(),
            "test_files/vault-kms-provider.yaml".to_string(),
        )
        .await
        .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn returns_not_found_if_no_matching_path_exists() {
        let resp = checks(
            "/invalid".to_string(),
            "test_files/vault-kms-provider.yaml".to_string(),
        )
        .await
        .unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}
