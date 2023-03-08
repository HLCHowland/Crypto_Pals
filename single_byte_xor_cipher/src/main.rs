extern crate hex;

fn main() {

    let cipher_text = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let mut cipher_bytes:Vec<u8> = Vec::new();
    let mut plain_text:Vec<u8> = Vec::new();
    let mut cipher:u8 = 1;

    match hex::decode(cipher_text){
        Ok(bytes) => cipher_bytes = bytes,
        Err(e) => println!("Failure to decode: {}", e),
    }

    while cipher < 255{
        for byte in &cipher_bytes {
            print!("{}",(byte ^ cipher) as char);
        }
        println!("Cipher: {}", cipher);
        cipher += 1;
    }
}

// cOOKINGmcSLIKEAPOUNDOFBACON