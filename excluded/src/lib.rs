use hex_literal::hex;

pub fn excluded() {
    println!("Excluded, hex = {:?}", hex!("1234"));
}
