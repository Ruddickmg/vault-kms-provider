use crate::configuration::authentication::Credentials;
use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt,
};
use notify::{
    event::{AccessKind, AccessMode},
    Event,
    EventKind::Access,
    RecommendedWatcher, RecursiveMode, Watcher,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_stream::StreamExt;
use tonic::async_trait;
use tracing::info;

pub fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);
    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Default::default(),
    )?;
    Ok((watcher, rx))
}

#[async_trait]
pub trait Refresh {
    async fn refresh_token(&mut self) -> Result<(), std::io::Error>;
}

pub async fn watch<T: Refresh>(
    path_to_watch: Option<String>,
    client: Arc<RwLock<T>>,
) -> Result<(), std::io::Error> {
    if let Some(path) = path_to_watch {
        let (mut watcher, mut rx) =
            async_watcher().map_err(|error| std::io::Error::other(error.to_string()))?;
        watcher
            .watch(path.as_ref(), RecursiveMode::NonRecursive)
            .map_err(|error| std::io::Error::other(error.to_string()))?;
        info!("Watching file at path: \"{}\" for updates", path);
        while let Some(Ok(event)) = rx.next().await {
            if event.kind == Access(AccessKind::Close(AccessMode::Write)) {
                event.paths.iter().for_each(|path| {
                    info!(
                        "Refreshing token due to updated JWT at path: {}",
                        path.to_str().unwrap()
                    )
                });
                client
                    .write()
                    .await
                    .refresh_token()
                    .await
                    .map_err(|e| std::io::Error::other(e.to_string()))?;
            }
        }
    }
    Ok(())
}

pub async fn watch_credentials<T: Refresh>(
    credentials: Credentials,
    client: Arc<RwLock<T>>,
) -> Result<(), std::io::Error> {
    watch(
        match credentials {
            Credentials::Kubernetes(credentials) => credentials.jwt.path(),
            Credentials::AppRole(role) => role.secret_id.path(),
            Credentials::Token(token) => token.path(),
            Credentials::UserPass(credentials) => credentials.password.path(),
            Credentials::Jwt(credentials) => credentials.jwt.path(),
            _ => None,
        },
        client,
    )
    .await
}

#[cfg(test)]
mod watcher {
    use super::*;
    use crate::configuration::authentication::{AppRole, Jwt, Kubernetes, UserPass};
    use crate::utilities::source::Source;
    use std::io::Error;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::sync::RwLock;
    use tonic::async_trait;
    use uuid::Uuid;

    struct Mock {
        called: bool,
    }

    impl Mock {
        pub fn new() -> Self {
            Self { called: false }
        }
    }

    #[async_trait]
    impl Refresh for Mock {
        async fn refresh_token(&mut self) -> Result<(), Error> {
            self.called = true;
            Ok(())
        }
    }

    async fn check_credential_path(credentials: Credentials, file_path: &str) {
        let mock_client = Arc::new(RwLock::new(Mock::new()));
        std::fs::write(file_path, "Hello World!").unwrap();
        tokio::select! {
            _ = async {
                watch_credentials(credentials, mock_client.clone()).await.unwrap();
            } => (),
            _ = async {
                tokio::time::sleep(Duration::from_millis(100)).await;
                std::fs::write(&file_path, "Goodbye Stranger!").unwrap();
                tokio::time::sleep(Duration::from_millis(100)).await;
            } => (),
        }
        assert!(mock_client.read().await.called);
    }

    mod watch_credentials {
        use super::*;
        use crate::configuration::authentication::Certificate;

        #[tokio::test]
        async fn watches_kubernetes_jwt_path() {
            let file_path = format!("./test_files/test-watch-file-{}", Uuid::new_v4());
            let credentials = Credentials::Kubernetes(Kubernetes::new(
                Source::FilePath(file_path.clone()),
                None,
                None,
            ));
            check_credential_path(credentials, &file_path).await;
        }

        #[tokio::test]
        async fn watches_jwt_path() {
            let file_path = format!("./test_files/test-watch-file-{}", Uuid::new_v4());
            let credentials =
                Credentials::Jwt(Jwt::new(Source::FilePath(file_path.clone()), None, None));
            check_credential_path(credentials, &file_path).await;
        }

        #[tokio::test]
        async fn watches_app_role_secret_id() {
            let file_path = format!("./test_files/test-watch-file-{}", Uuid::new_v4());
            let credentials = Credentials::AppRole(AppRole::new(
                "role_id".to_string(),
                Source::FilePath(file_path.clone()),
                None,
            ));
            check_credential_path(credentials, &file_path).await;
        }

        #[tokio::test]
        async fn watches_password() {
            let file_path = format!("./test_files/test-watch-file-{}", Uuid::new_v4());
            let credentials = Credentials::UserPass(UserPass::new(
                "password".to_string(),
                Source::FilePath(file_path.clone()),
                None,
            ));
            check_credential_path(credentials, &file_path).await;
        }

        #[tokio::test]
        async fn watches_token() {
            let file_path = format!("./test_files/test-watch-file-{}", Uuid::new_v4());
            let credentials = Credentials::Token(Source::FilePath(file_path.clone()));
            check_credential_path(credentials, &file_path).await;
        }

        #[tokio::test]
        async fn does_not_watch_credentials_with_no_path() {
            let mock_client = Arc::new(RwLock::new(Mock::new()));
            let credentials = Credentials::Certificate(Certificate::new("cert".to_string(), None));
            let result = watch_credentials(credentials, mock_client).await;
            assert!(result.is_ok());
        }
    }

    mod watch {
        use super::*;

        #[tokio::test]
        async fn refreshes_token_when_file_changes() -> Result<(), Box<dyn std::error::Error>> {
            let path = "./test_files/test_watched_file";
            let mock_client = Arc::new(RwLock::new(Mock::new()));
            std::fs::write(&path, "Hello World!").unwrap();
            tokio::select! {
                _ = async {
                    watch(Some(path.to_string()), mock_client.clone()).await.unwrap();
                    Ok::<(), std::io::Error>
                } => (),
                _ = async {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                    std::fs::write(&path, "Goodbye Stranger!").unwrap();
                    tokio::time::sleep(Duration::from_millis(100)).await;
                    Ok::<(), std::io::Error>(())
                } => (),
            }
            assert!(mock_client.read().await.called);
            Ok(())
        }
    }
}
