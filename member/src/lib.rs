use excluded::excluded;
use hex_literal::hex;

pub fn member() {
    excluded();
    println!("Member, hex = {:?}",hex!("00"));
}
