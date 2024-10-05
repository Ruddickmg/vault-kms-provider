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

#[cfg(test)]
mod watch {
    use super::*;
    use std::io::Error;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::sync::RwLock;
    use tonic::async_trait;

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
                tokio::time::sleep(Duration::from_millis(1)).await;
                std::fs::write(&path, "Goodbye Stranger!").unwrap();
                tokio::time::sleep(Duration::from_millis(1)).await;
                Ok::<(), std::io::Error>(())
            } => (),
        }
        assert!(mock_client.read().await.called);
        Ok(())
    }
}
