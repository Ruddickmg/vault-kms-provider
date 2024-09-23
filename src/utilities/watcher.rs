use crate::vault::Client;
use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt,
};
use notify::event::{AccessKind, AccessMode};
use notify::EventKind::Access;
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_stream::StreamExt;
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

pub async fn watch(
    path_to_watch: Option<String>,
    client: Arc<RwLock<Client>>,
) -> Result<(), std::io::Error> {
    if let Some(path) = path_to_watch {
        let (mut watcher, mut rx) = async_watcher().unwrap();
        watcher
            .watch(path.as_ref(), RecursiveMode::NonRecursive)
            .unwrap();
        while let Some(res) = rx.next().await {
            if let Ok(mut event) = res {
                if event.kind == Access(AccessKind::Close(AccessMode::Write)) {
                    info!(
                        "Refreshing token due to updated JWT at path: {}",
                        event.paths.pop().unwrap().to_str().unwrap()
                    );
                    client
                        .write()
                        .await
                        .refresh_token()
                        .await
                        .map_err(|e| std::io::Error::other(e.to_string()))?;
                }
            }
        }
    }
    Ok(())
}
