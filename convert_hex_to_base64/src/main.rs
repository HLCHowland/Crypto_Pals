extern crate hex;
extern crate base64;

fn main() {
    let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    match hex::decode(hex_string) {
        // Ok(bytes) => println!("{:?}", bytes),
        Ok(bytes)=> println!("{}", base64::encode(&bytes)),
        Err(e) => println!("Error decoding hex string: {}", e),
    }
}
