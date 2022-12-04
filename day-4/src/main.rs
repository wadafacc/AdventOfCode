use std::collections::btree_map::Range;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open("./data.txt").unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let x: Vec<&str> = line.split("-").collect();
        let y = x.join(",");
        let rr: Vec<&str> = y.split(",").collect();
        let mut r: Vec<i32> = Vec::new();
        for i in rr {
            r.push(i.parse::<i32>().unwrap());
        }
        println!("{:?}", r);
        // 3,4,5,6
        if (r[0] <= r[2] && r[1] >= r[3]) || (r[0] >= r[2] && r[1] <= r[3]) {
            sum += 1;
        }
    }
    println!("{:?}", sum);
}
