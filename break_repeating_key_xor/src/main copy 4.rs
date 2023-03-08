use std::fs::File;
use std::io::{BufRead, BufReader};
extern crate hex;
use base64;


// // computes the hamming weight between two strings of identical length 
// // hammking weight aka edit distance
fn get_edit_dist(string_1:&str, string_2:&str) -> i32{
    if string_1.len() != string_2.len(){
        return -1 
    }
    let bytes_1 = string_1.as_bytes();
    let bytes_2 = string_2.as_bytes();
    let mut byte_xor = vec![0; string_1.len()];
    for i in 0..string_1.len(){
        byte_xor[i] = bytes_1[i] ^ bytes_2[i];
    }
    let mut xor_sum:i32 = 0;
    for byte in byte_xor{
        xor_sum += (byte.count_ones()) as i32;
    }
    return xor_sum;
}


fn main() {
    let mut cipher_bytes:Vec<u8> = Vec::new();    
    let mut keysize = 0;
    let file = File::open("/home/casper/Crypto/crypto_pals/break_repeating_key_xor/src/6.txt").expect("Failed to open file.");
    let reader = BufReader::new(file);
    let mut byte_line_vec:Vec<Vec<u8>> = Vec::new();

    let it = 0;

    for line in reader.lines() {
        println!("IT: {}\n", it);
        let b64_line = line.unwrap();
        let hex_line = base64::decode(b64_line).unwrap();
        let byte_line = hex::decode(hex_line).unwrap();
        byte_line_vec.push(byte_line);
    }
    for byte_line in byte_line_vec{
        let bytes_1 = &byte_line[0..keysize];
        let bytes_2 = &byte_line[keysize..2*keysize];
        let mut xor_bytes = vec![0; keysize];
        for i in 0..keysize{
            xor_bytes[i] = bytes_1[i] ^ bytes_2[i];
        }
        let mut xor_sum:i32 = 0;
        for byte in xor_bytes{
            xor_sum += (byte.count_ones()) as i32;
        }
        println!("Xor Sum: {}",xor_sum);
    }
}
