use tracing::{error, info};

pub fn log_request(message: &str) {
    info!("{}", message);
}

pub fn log_error(message: &str) {
    error!("{}", message);
}
