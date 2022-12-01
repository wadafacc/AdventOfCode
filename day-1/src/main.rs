use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open("./data.txt").unwrap();
    let reader = BufReader::new(file);

    // init some stuff
    let mut max = 0;
    let mut sum = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if line == "" {
            // skip line & make new dwarf
            if sum >= max {
                max = sum;
            }
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }
    println!("Max: {:?}", max);
}
