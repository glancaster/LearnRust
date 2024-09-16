use log::*;

fn main() {
    // Create a logger to manage logging level.
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)  // Level set to Info.
        .init();

    // Logging levels from less to most severe.
    trace!("trace message");
    debug!("debug message");
    info!("info message");
    warn!("warn message");
    error!("error message");
}
