extern crate hex;

fn main() {
    let hex_string_1 = "1c0111001f010100061a024b53535009181c";
    let hex_string_2 = "686974207468652062756c6c277320657965";
    let mut bytes_1:Vec<u8> = Vec::new();
    let mut bytes_2:Vec<u8> = Vec::new();
    let mut bytes_out:Vec<u8> = Vec::new();
    match hex::decode(hex_string_1) {
        Ok(bytes)=> bytes_1 = bytes,
        Err(e) => println!("Error decoding hex string: {}", e),
    }
    match hex::decode(hex_string_2) {
        Ok(bytes)=> bytes_2 = bytes,
        Err(e) => println!("Error decoding hex string: {}", e),
    }
    for (x, y) in bytes_1.iter().zip(bytes_2.iter()){
        bytes_out.push(x ^ y);
    }
    println!("{}", hex::encode(bytes_out));
}



