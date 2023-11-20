use color_eyre::eyre::Result;
use tracing::{debug, error, info, trace, warn, Level};

#[test]
fn test() {
    // tracing_subscriber::fmt::init(); //default Level::INFO
    tracing_subscriber::fmt()
        .pretty()
        .with_thread_names(true)
        .with_max_level(Level::TRACE)
        .init();
    // color_eyre::install().unwrap();

    trace!("log-trace");
    debug!("log-debug");
    info!("log-info");
    warn!("loog-warn");
    error!("log-error");
}
