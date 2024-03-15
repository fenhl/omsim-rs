use std::fs::File;
use std::io::Read;
use crate::parse::{parse_puzzle, parse_solution};

mod parse;
mod data;

fn main() {
    let mut buffer: Vec<u8> = Vec::new();
    let _ = File::open("./OM2023Weeklies_DarkMatterCandidate.puzzle").unwrap().read_to_end(&mut buffer).unwrap();
    let puzzle = parse_puzzle(buffer.as_slice()).unwrap();
    println!("it's like {:?}", puzzle);

    println!("and then!!");

    let mut buffer2: Vec<u8> = Vec::new();
    let _ = File::open("./dental-amalgam-OM2024_W4_Dental_Amalgam-34.solution").unwrap().read_to_end(&mut buffer2).unwrap();
    let sol = parse_solution(buffer2.as_slice()).unwrap();
    println!("they're like {:?}", sol);
}
