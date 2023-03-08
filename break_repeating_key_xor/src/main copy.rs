use std::fs::File;
use std::io::{BufRead, BufReader};
extern crate hex;

// computes the hamming weight between two strings of identical length 
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

fn main() {


    let string_1 = "this is a test";
    let string_2 = "wokka wokka!!!";
    let edit_dist = get_edit_dist(string_1, string_2);
    println!("Edit Distance {}", edit_dist);


    let mut cipher_bytes:Vec<u8> = Vec::new();    
    let mut keysize = 0;
    let file = File::open("/home/casper/Crypto/crypto_pals/break_repeating_key_xor/src/6.txt").expect("Failed to open file.");
    let reader = BufReader::new(file);


    // for cipher_text  in reader.lines() {
    //     match hex::decode(cipher_text.unwrap()){
    //         Ok(bytes) => cipher_bytes = bytes,
    //         Err(e) => println!("Failure to decode: {}", e),
    //     }
    //     // need to take keysize bytes and the second keysize bytes
    //     // pretty sure keysize must be less than 30. 
    //     // need to put the first x bytes into a string and the next x bytes into anotehr 
    //     let alt_byte_1 = cipher_bytes[0..keysize];
    //     let alt_byte_2 = cipher_bytes[keysize..2*keysize];
    // }    
}
