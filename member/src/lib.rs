use excluded::setup;
use hex_literal::hex;

pub fn f() {
    setup();
    println!("Hello, world!");
    println!("{:?}",hex!("00"));
}
