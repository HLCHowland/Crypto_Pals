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

    let mut hex_vec = Vec::new();

// ughhhh need to base64 decode first, think that was always the problem 
//////////////////////////////////////////////////////

// okay lol, I have not done a base64 decode, that's why i cannot find it 


// HERE IS HOW THE BELOW LINES WORK
// line? - This is a shorthand way of checking if line is an Ok result or an Err result. If line is an Err result, the ? operator will return the error early and the function will exit with an error. If line is an Ok result, the ? operator will unwrap the Ok value and continue to the next operation.
// .trim() - This is a method on a String that removes whitespace from both ends of the string. In this case, it's used to remove any whitespace at the beginning or end of line.
// decode() - This is a function from the base64 crate that decodes a Base64-encoded string into bytes. It takes a &[u8] (a slice of bytes) as input and returns a Result<Vec<u8>, base64::DecodeError>.
// ? - This is the same operator as in step 1. If decode() returns an Err result, the ? operator will return the error early and the function will exit with an error. If decode() returns an Ok result, the ? operator will unwrap the Ok value and assign it to base64_decoded.




    for line in reader.lines(){
        let base64_decoded = base64::decode(line?.trim())?;    
        let hex = hex::decode(base64_decoded?.trim())?;
        hex_vec.push(hex);
    }
    

    // for cipher_text  in reader.lines() {
    //     match hex::decode(cipher_text.unwrap()){
    //         Ok(bytes) => cipher_bytes = bytes,
    //         Err(e) => println!("Failure to decode: {}", e),
    //     }
        // Need to rwrite this to decode correctly 


        // // Have to use references here because we don't know how big the keysize will be at compiletime 
        // let substring_1 = &cipher_bytes[0..keysize];
        // let substring_2 = &cipher_bytes[keysize..2*keysize];

        // let string_1 = std::str::from_utf8(substring_1).unwrap();
        // let string_2 = std::str::from_utf8(substring_2).unwrap();
        // let xor_sum = get_edit_dist(string_1, string_2);
        // println!("XOR Sum: {}", xor_sum);
    // }    
}
