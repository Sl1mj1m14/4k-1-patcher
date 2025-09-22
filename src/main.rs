use std::{fs::{read, OpenOptions}, io::{Read,Write}};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

const BYTE0: u8 = 0x1F;
const BYTE1: u8 = 0x8B;

pub fn main() {

    let mut path: String = "".to_string();
    println!("Please paste the path to your file");
    std::io::stdin().read_line(&mut path).expect("And this somehow broke...");
    path = path.trim().to_string();

    let stream: Vec<u8> = read(&path).unwrap();
    let mut bytes: Vec<u8> = Vec::new();

    if stream[0] == BYTE0 && stream[1] == BYTE1 {
        let mut d_stream = GzDecoder::new(&stream[..]);
        d_stream.read_to_end(&mut bytes).unwrap();
    } else {
        bytes = stream.clone();
    }

    let mut bytes1: Vec<u8> = Vec::new();
    let mut n: i32 = 0;
    for byte in bytes {
        if byte > 15 {bytes1.push(byte)}
        else {bytes1.push(1); n += 1;}
    }

    println!{"Patched out {} bytes greater than 15", n};

    let mut version: u8 = 3;
        while version > 2 {
            let mut buf = "".to_string();
            println!("Which 4k version would you like to output the patched file to?");
            println!("[4k-011742] Type 0\n[4k-040144] Type 1\n[4k-javascript] Type 2");
            std::io::stdin().read_line(&mut buf).expect("And this somehow broke...");
            version = buf.trim().parse().unwrap();

            if version > 2 {println!("You absolute brainless idiot, all I asked you to do was to type one god forsaken number. You literally only had to count to 2 you moron. An infant would perform better than you. Go die in a corner. Enjoy suffering in Hell.")}
        }

    let mut output= OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap();

    if version != 2 {
        let mut encoder = GzEncoder::new(output, Compression::default());
        encoder.write_all(&bytes1).unwrap();
    } else {
        output.write(&bytes1).expect("And this somehow broke...");
    }
}