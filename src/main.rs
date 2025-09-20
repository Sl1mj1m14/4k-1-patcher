use std::{fs::{read, OpenOptions}, io::{Read,Write}};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

pub fn main() {

    let mut path: String = "".to_string();
    println!("Please paste the path to your file");
    std::io::stdin().read_line(&mut path).expect("And this somehow broke...");
    path = path.trim().to_string();

    let stream: Vec<u8> = read(&path).unwrap();
    let mut bytes: Vec<u8> = Vec::new();
    let mut d_stream = GzDecoder::new(&stream[..]);
    d_stream.read_to_end(&mut bytes).unwrap();

    let mut bytes1: Vec<u8> = Vec::new();
    let mut n: i32 = 0;
    for byte in bytes {
        if byte > 15 {bytes1.push(byte)}
        else {bytes1.push(1); n += 1;}
    }

    let mut output= OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap();

    let mut encoder = GzEncoder::new(output, Compression::default());
    encoder.write_all(&bytes1).unwrap();
    println!{"Patched out {} bytes greater than 15", n};

}