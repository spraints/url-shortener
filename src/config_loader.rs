use std::fmt::Debug;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use notify::RecommendedWatcher;
use notify::{Event, RecursiveMode, Watcher};
use tokio::sync::mpsc;
use tracing::{debug, error, info};

use crate::state::AppState;

pub fn watch<P>(state: Arc<AppState>, config_path: P) -> anyhow::Result<RecommendedWatcher>
where
    P: AsRef<Path> + Debug,
{
    let (tx, mut rx) = mpsc::channel::<Result<Event, notify::Error>>(100);

    let mut watcher = notify::recommended_watcher(move |res| {
        debug!("GOT FS EVENT");
        // Send the event through the channel
        if let Err(e) = tx.blocking_send(res) {
            error!("failed to send fs event: {e}");
        }
    })?;

    let config_path = config_path.as_ref();
    debug!("watching {config_path:?} for changes");
    watcher.watch(config_path, RecursiveMode::NonRecursive)?;

    let config_path = config_path.to_path_buf();
    tokio::spawn(async move {
        load_config(&state, &config_path).await;
        while let Some(e) = rx.recv().await {
            match e {
                Err(e) => error!("error watching {config_path:?}: {e}"),
                Ok(_) => load_config(&state, &config_path).await,
            };
        }
    });

    Ok(watcher)
}

async fn load_config(state: &AppState, config_path: &PathBuf) {
    info!("reloading config from {config_path:?}");
    if let Err(e) = try_load_config(&state, config_path).await {
        error!("error loading config from {config_path:?}: {e}");
    }
}

async fn try_load_config<P: AsRef<Path>>(state: &AppState, config_path: P) -> anyhow::Result<()> {
    let data = tokio::fs::read(config_path).await?;
    state.set_config(serde_yaml::from_slice(&data)?).await;
    Ok(())
}
