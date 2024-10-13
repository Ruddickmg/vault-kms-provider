use bytes::Bytes;
use http::{Response, StatusCode};
use http_body_util::Full;
use std::convert::Infallible;
use std::path::Path;

pub async fn readiness_check(socket_path: &str) -> Result<Response<Full<Bytes>>, Infallible> {
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

#[cfg(test)]
mod readiness {
    use super::readiness_check;
    use http::StatusCode;

    #[tokio::test]
    async fn returns_ok_if_socket_exists() {
        let resp = readiness_check("test_files/vault-kms-provider.yaml")
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn returns_error_status_if_socket_does_not_exist() {
        let resp = readiness_check("test_files/non-existent-file")
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
