use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
extern crate hex;

fn setup_hashmap() -> HashMap<char, f32>{
    let mut freq_map: HashMap<char, f32> = HashMap::new();
    freq_map.insert('E', 12.70);
    freq_map.insert('T', 9.06);
    freq_map.insert('A', 8.17);
    freq_map.insert('O', 7.51);
    freq_map.insert('I', 6.97);
    freq_map.insert('N', 6.75);
    freq_map.insert('S', 6.33);
    freq_map.insert('H', 6.09);
    freq_map.insert('R', 5.99);
    freq_map.insert('D', 4.25);
    freq_map.insert('L', 4.03);
    freq_map.insert('U', 2.76);
    freq_map.insert('C', 2.78);
    freq_map.insert('M', 2.41);
    freq_map.insert('F', 2.23);
    freq_map.insert('Y', 2.02);
    freq_map.insert('W', 2.36);
    freq_map.insert('G', 2.02);
    freq_map.insert('P', 1.93);
    freq_map.insert('B', 1.29);
    freq_map.insert('V', 0.98);
    freq_map.insert('K', 0.77);
    freq_map.insert('X', 0.15);
    freq_map.insert('Q', 0.10);
    freq_map.insert('J', 0.15);
    freq_map.insert('Z', 0.07);    
    return freq_map;
}

pub struct ScoreEntry {
    score: f32,
    xor: u8,
    pt: String,
}

fn main() {
    let file = File::open("/home/casper/Crypto/crypto_pals/chal_4/src/4.txt").expect("Failed to open file.");
    let reader = BufReader::new(file);
    let mut freq_map: HashMap<char, f32> = setup_hashmap();
    let mut x = 1;
    let mut freq_sum: f32 = 0.0;
    // for cipher_text in reader.lines() {
    for l in reader.lines() {
        if x > 1{break}else{x += 1;}  //Just run once       

// testing with a string we know is correct 
        let cipher_text = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

        let mut cipher_bytes:Vec<u8> = Vec::new();
        let mut plain_text:Vec<u8> = Vec::new();
        let mut cipher:u8 = 1;

        // Convert the cipher text to its byte representation 
        // match hex::decode(cipher_text.unwrap()){
        
        match hex::decode(cipher_text){
            Ok(bytes) => cipher_bytes = bytes,
            Err(e) => println!("Failure to decode: {}", e),
        }
        
        let mut xor: u8 = 0;
        while cipher < 255{
            let mut plain_text: String = String::new();
            for byte in &cipher_bytes {
                xor = *byte;
                let tmp_char = ((byte ^ cipher) as char).to_ascii_uppercase();
                match freq_map.get(&tmp_char){
                    Some(freq) => freq_sum += freq,
                    None => {},
                }
                plain_text.push(tmp_char);
            }

            // println!("{}", plain_text);

            // println!("{}",String::from_utf8(plain_text).unwrap());
            println!("{}",plain_text);

            

            let  score_entry = ScoreEntry  {
                score: freq_sum,
                xor: xor,
                pt: plain_text
            };

            println!("\nCipher: {}", cipher);
            println!("Freq: {}", freq_sum);
            freq_sum = 0.0;
            cipher += 1;
        }
    }
}
