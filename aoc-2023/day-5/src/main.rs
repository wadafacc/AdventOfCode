use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Instant;

fn main() {
    // bench
    let bench = Instant::now();

    let file = File::open("./data.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l|l.expect("cant read lad")).collect();
    
    let seeds:Vec<&str> = lines[0].split_once(":").unwrap().1.split_ascii_whitespace().collect();

    lines.iter().enumerate().for_each(|(_i,line)| {

        
    });

    // more bench
    println!("Elapsed time: {:.2?}", bench.elapsed());
}