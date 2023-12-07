use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Instant;

fn main() {
    // bench
    let bench = Instant::now();

    let file = File::open("./data.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l|l.expect("cant read lad")).collect();
    
    let time:Vec<i32> = lines[0].split_once(":").unwrap().1.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let records:Vec<i32> = lines[1].split_once(":").unwrap().1.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut beat_record: Vec<i32> = Vec::new();

    for i in 0..records.len() {
        beat_record.push(test_combinations(records[i], time[i]));
    }

    let mut total = 1;
    beat_record.iter().for_each(|n| total *= n );


    println!("{:?} | TOTAL: {}", beat_record, total);

    // more bench
    println!("Elapsed time: {:.2?}", bench.elapsed());
}

fn test_combinations(record:i32, time:i32) -> i32 {
    let mut count = 0;

    for button_held in 0..=time {
            if (time - button_held) * button_held > record {
                count += 1;
            }
    }   

    return count;
}