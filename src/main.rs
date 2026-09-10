use std::fmt;
use std::io::{Error, ErrorKind};
use std::path::Path;
use tracing::{debug, error, info, warn};

mod ipc;
mod notify;

const NUM_CAMS: usize = 8;
const CHECK_INTERVAL_SECS: u64 = 60;

#[derive(Debug)]
enum WatchError {
    NotFound,
    Io(Error),
    SomeMissing,
    TimeError(std::time::SystemTimeError),
}

impl fmt::Display for WatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WatchError::Io(error) => write!(f, "Io error: {error}"),
            WatchError::NotFound => write!(f, "Not Found"),
            WatchError::SomeMissing => write!(f, "Some missing"),
            WatchError::TimeError(error) => write!(f, "Time error: {error}"),
        }
    }
}

impl From<std::time::SystemTimeError> for WatchError {
    fn from(error: std::time::SystemTimeError) -> Self {
        WatchError::TimeError(error)
    }
}

impl From<Error> for WatchError {
    fn from(error: Error) -> Self {
        WatchError::Io(error)
    }
}

impl std::error::Error for WatchError {}

fn check_recs(hourly_dir_path: &Path) -> Result<(), WatchError> {
    let mut cams = 0;
    // if NFS is not mounted, read_dir returns error here
    let readdir = std::fs::read_dir(hourly_dir_path)?;
    // folder could still be empty because we checked only a few seconds after :00
    // in this case no error but readdir is empty, Some Missing error returned below
    for cdir in readdir {
        let cdir = cdir?;
        debug!("checking dir: {:?}", cdir.file_name());
        let mut creations: Vec<_> = std::fs::read_dir(cdir.path())?
            .filter_map(|direntry| direntry.ok())
            .filter_map(|entry| entry.metadata().ok())
            // created is not available in a docker container, prefer modified
            .filter_map(|m| m.modified().ok())
            .collect();

        creations.sort();

        let latest = creations.last().ok_or(WatchError::NotFound)?;

        let now = std::time::SystemTime::now();
        let time_passed = now.duration_since(*latest)?;

        if time_passed > std::time::Duration::from_mins(1) {
            return Err(WatchError::NotFound);
        }

        cams += 1;
    }

    if cams != NUM_CAMS {
        return Err(WatchError::SomeMissing);
    }
    Ok(())
}

fn watchnfs(path: &str) -> std::io::Result<()> {
    // The folder structure for the recordings is YYYY-MM-DD/HH/<camera_name>/MM.SS.mp4 in UTC time.
    // https://docs.frigate.video/configuration/record

    let now = chrono::Utc::now();
    let path = format!("{}/{}/{}", path, now.format("%Y-%m-%d"), now.format("%H"));
    let hourly_dir = Path::new(&path);

    for _ in 1..10 {
        match check_recs(hourly_dir) {
            Ok(_) => return Ok(()),
            // could be a timing issue -- retry
            Err(WatchError::NotFound) => {
                warn!("Not found -- retrying in 10 seconds");
            }
            // maybe right after :00, directory exists but still empty
            Err(WatchError::SomeMissing) => {
                warn!("Some missing -- retrying in 10 seconds");
            }
            // critical errors
            Err(WatchError::Io(error)) => return Err(error),
            Err(WatchError::TimeError(_)) => {
                return Err(Error::other("should not happen -- time conversion error"))
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
    Err(Error::new(
        ErrorKind::NotFound,
        "recordings not found after retry",
    ))
}

#[tokio::main]
async fn main() {
    let timer = tracing_subscriber::fmt::time::ChronoLocal::rfc_3339();
    tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_timer(timer)
        .init();

    dotenv::dotenv().ok();
    let rec_path = dotenv::var("NFS_REC_PATH").expect("NFS_REC_PATH must be set in .env");

    let handle = tokio::spawn(async move {
        loop {
            match watchnfs(&rec_path) {
                Ok(_) => info!("recordings are working fine"),
                Err(e) => {
                    error!("{}", e);
                    notify::send_mail().await;
                    let res =
                        tokio::spawn(ipc::send_alarm_notification(ipc::IpcMessage::MakeNoise));
                    if res.await.is_err() {
                        error!("failed to send message notification to olympus");
                    }
                }
            };
            std::thread::sleep(std::time::Duration::from_secs(CHECK_INTERVAL_SECS));
        }
    });

    let _ = handle.await;
}
