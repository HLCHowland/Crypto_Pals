use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

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


// Need it to go through each line and find the xor with the highest value 
// then go through each one of those, doing another frequency analysis to see which is th emost likely to be a real sentence...

fn main() {
    let file = File::open("/home/casper/Crypto/crypto_pals/chal_4/src/4.txt").expect("Failed to open file.");
    let reader = BufReader::new(file);
    let mut freq_map: HashMap<char, f32> = setup_hashmap();
    let mut freq_sum: f32 = 0.0;
    let mut x = 1;
    for line in reader.lines() {
        // if x > 1{break}else{x += 1;}  //Just run once       
        // need to select the highest frequency after decoding for each 


        for c in line.unwrap().chars(){
            match freq_map.get(&c.to_ascii_uppercase()) {
                Some(freq) => freq_sum += freq,
                None => {},
            }
        }
        println!("{}", freq_sum);
        freq_sum = 0.0;

    }



}
