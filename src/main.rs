use std::fs::File;
use std::io::Read;
use crate::parse::parse_puzzle;

mod parse;
mod data;

fn main() {
    let mut buffer: Vec<u8> = Vec::new();
    let _ = File::open("./OM2023Weeklies_DarkMatterCandidate.puzzle").unwrap().read_to_end(&mut buffer).unwrap();
    let puzzle = parse_puzzle(buffer.as_slice()).unwrap();
    println!("it's like {:?}", puzzle)
}
