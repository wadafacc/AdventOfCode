use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Instant;

fn main() {
    // bench
    let bench = Instant::now();

    let file = File::open("./data.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l|l.expect("cant read lad")).collect();
    
    let time:String = lines[0].split_once(":").unwrap().1.split_ascii_whitespace().collect();
    let records:String = lines[1].split_once(":").unwrap().1.split_ascii_whitespace().collect();

    println!("{:?}", test_combinations(records.parse::<i128>().unwrap(), time.parse::<i128>().unwrap()));

    // more bench
    println!("Elapsed time: {:.2?}", bench.elapsed());
}

fn test_combinations(record:i128, time:i128) -> i32 {
    let mut count = 0;

    for button_held in 0..=time {
            if (time - button_held) * button_held > record {
                count += 1;
            }
    }   

    return count;
}