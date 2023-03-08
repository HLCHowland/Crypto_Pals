extern crate hex;
fn main() {
    let pt = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = "ICE"; //need to convert this to something I can iterate over 
    let mut bytes_out:Vec<u8> = Vec::new();
    let mut xori = 0;
    let mut xorb = 0;
    for byte in pt.chars(){
        if xori ==3{
            xori = 0;
        }
        xorb: u8 = key.as_bytes()[xori];
        bytes_out.push(xorb ^ byte as u8);
        xori += 1;
    }        
    println!("{}", hex::encode(bytes_out));
}

