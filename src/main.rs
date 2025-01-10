use anyhow::{anyhow, Context, Result};
use chrono::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

fn check(path: &Path, num_of_recs: usize) -> Result<()> {
    path.try_exists().context("Nfs not mounted!")?;
    // The folder structure for the recordings is YYYY-MM-DD/HH/<camera_name>/MM.SS.mp4 in UTC time.
    // https://docs.frigate.video/configuration/record
    let expected_directory = Utc::now().format("%Y-%m-%d").to_string();
    let daily_dir = path.join(expected_directory);
    daily_dir
        .try_exists()
        .context("directory of the day does not exist")?;
    let hour = Utc::now().hour();
    let hourly_dir = daily_dir.join(hour.to_string());
    hourly_dir
        .try_exists()
        .context("directory of the hour does not exist")?;

    let rec_dirs: Vec<PathBuf> = fs::read_dir(hourly_dir)
        .context("Unable to read contents of hourly dir")?
        .map(|i| i.unwrap().path())
        .collect();

    if rec_dirs.len() == num_of_recs {
        Ok(())
    } else {
        Err(anyhow!("recordings missing!"))
    }
}

fn main() {
    //TODO: update path
    let path = Path::new("/mnt/diskos/recordings");
    let num_of_recs = 8;

    let join_handle = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(5));
            match check(path, num_of_recs) {
                Ok(_) => println!("recordings are working fine."),
                Err(e) => {
                    //TODO: send notification
                    println!("{}", e);
                }
            }
        }
    });

    join_handle.join().expect("Panic when joining thread");
}
