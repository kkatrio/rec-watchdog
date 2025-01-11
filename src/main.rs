use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use std::path::{Path, PathBuf};
use tracing::{info, warn};
use tracing_subscriber;

mod notify;

fn check(path: &Path, num_of_recs: usize) -> Result<()> {
    // The folder structure for the recordings is YYYY-MM-DD/HH/<camera_name>/MM.SS.mp4 in UTC time.
    // https://docs.frigate.video/configuration/record
    let now = Utc::now();
    let rel_path = format!("{}/{}", now.format("%Y-%m-%d"), now.format("%H"));
    let hourly_dir = path.join(rel_path);

    // returns error if no directory has been created
    let rec_dirs: Vec<PathBuf> = std::fs::read_dir(&hourly_dir)
        .with_context(|| format!("No hourly dir: {:?}!", &hourly_dir))?
        .map(|i| i.unwrap().path())
        .collect();

    if rec_dirs.len() == num_of_recs {
        Ok(())
    } else {
        Err(anyhow!("recordings missing! call Kon."))
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    //TODO: update path
    let path = Path::new("/mnt/diskos/recordings");
    let num_of_recs = 8;

    tokio::spawn(async move {
        //TODO: raise alarm every second, send notifications evry 60 secs.
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(600));
        loop {
            interval.tick().await;
            match check(path, num_of_recs) {
                Ok(_) => info!("recordings are working fine."),
                Err(e) => {
                    warn!("{}", e);
                    notify::send_mail().await;
                }
            }
        }
    });

    tokio::signal::ctrl_c()
        .await
        .context("signal::ctrl_c failed to terminate.")
        .unwrap();
}
