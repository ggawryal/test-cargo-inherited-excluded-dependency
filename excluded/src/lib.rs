extern crate log;

use log::info;
use hex_literal::hex;

pub fn setup() {
    env_logger::init();
    info!("Logging enabled {:?}", hex!("1234"));
}
