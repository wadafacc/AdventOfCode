use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open("./data.txt").unwrap();
    let reader = BufReader::new(file);

    // init some stuff
    let mut sum = 0;
    let mut list: Vec<i32> = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if line == "" {
            list.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }

    list.sort();
    let max = list[list.len() - 1] + list[list.len() - 2] + list[list.len() - 3];
    println!("Max: {:?}", max);
}
