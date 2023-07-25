extern crate log;

use log::info;

pub fn setup() {
    env_logger::init();
    info!("Logging enabled");
}
