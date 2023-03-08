use std::fs::File;
use std::io::{BufRead, BufReader};
extern crate hex;

// // computes the hamming weight between two strings of identical length 
// // hammking weight aka edit distance
fn get_edit_dist(string_1:&str, string_2:&str) -> i32{
    if string_1.len() != string_2.len(){
        return -1 
    }
    let bytes_1 = string_1.as_bytes();
    let bytes_2 = string_2.as_bytes();
    // let mut byte_xor = Vec::with_capacity(0,string_1.len());
    let mut byte_xor = vec![0; string_1.len()];
    // for (byte_1, byte_2) in bytes_1.iter().zip(bytes_2.iter())
    for i in 0..string_1.len(){
        byte_xor[i] = bytes_1[i] ^ bytes_2[i];
    }
    let mut xor_sum:i32 = 0;
    for byte in byte_xor{
        xor_sum += (byte.count_ones()) as i32;
    }
    return xor_sum;
}

// computes the hamming weight between two strings of identical length 
// hammking weight aka edit distance
// fn get_edit_dist(string_1:Vec<u8>, string_2:Vec<u8>) -> i32{
//     if string_1.len() != string_2.len(){
//         return -1 
//     }
//     // let bytes_1 = string_1.as_bytes();
//     // let bytes_2 = string_2.as_bytes();
//     // let mut byte_xor = Vec::with_capacity(0,string_1.len());
//     let mut byte_xor = vec![0; string_1.len()];
//     // for (byte_1, byte_2) in bytes_1.iter().zip(bytes_2.iter())
//     for i in 0..string_1.len(){
//         byte_xor[i] = bytes_1[i] ^ bytes_2[i];
//     }
//     let mut xor_sum:i32 = 0;
//     for byte in byte_xor{
//         xor_sum += (byte.count_ones()) as i32;
//     }
//     return xor_sum;
// }

fn main() {
    let mut cipher_bytes:Vec<u8> = Vec::new();    
    let mut keysize = 0;

    let file = File::open("/home/casper/Crypto/crypto_pals/break_repeating_key_xor/src/6.txt").expect("Failed to open file.");
    let reader = BufReader::new(file);

    for cipher_text  in reader.lines() {
        match hex::decode(cipher_text.unwrap()){
            Ok(bytes) => cipher_bytes = bytes,
            Err(e) => println!("Failure to decode: {}", e),
        }
        // Have to use references here because we don't know how big the keysize will be at compiletime 
        let substring_1 = &cipher_bytes[0..keysize];
        let substring_2 = &cipher_bytes[keysize..2*keysize];

        let string_1 = std::str::from_utf8(substring_1).unwrap();
        let string_2 = std::str::from_utf8(substring_2).unwrap();
        let xor_sum = get_edit_dist(string_1, string_2);
        println!("XOR Sum: {}", xor_sum);
    }    
}
