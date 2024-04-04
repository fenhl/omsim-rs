use std::env;
use std::fs::File;
use std::io::Read;
use omsim_rs::parse::{parse_puzzle, parse_solution};
use omsim_rs::sim::Sim;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut buffer: Vec<u8> = Vec::new();
    let _ = File::open(&args[1]).unwrap().read_to_end(&mut buffer).unwrap();
    let puzzle = parse_puzzle(buffer.as_slice()).unwrap();
    println!("it's like {puzzle:?}\n");

    println!("and then!!");

    let mut buffer2: Vec<u8> = Vec::new();
    let _ = File::open(&args[2]).unwrap().read_to_end(&mut buffer2).unwrap();
    let sol = parse_solution(buffer2.as_slice()).unwrap();
    println!("they're like {sol:?}\n");

    let sim: Sim = Sim::create(&puzzle, &sol).unwrap();
    println!("and now we set the empty stage of {sim:?}");
}
