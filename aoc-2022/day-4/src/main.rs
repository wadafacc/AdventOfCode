use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open("./data.txt").unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let x: Vec<&str> = line.split("-").collect();
        let y = x.join(",");
        let rr: Vec<&str> = y.split(",").collect();
        let mut r: Vec<i32> = Vec::new();
        for i in rr {
            r.push(i.parse::<i32>().unwrap());
        }

        let mut yay: Vec<i32> = Vec::new();
        for i in r[0]..=r[1] {
            yay.push(i);
        }
        let mut aya: Vec<i32> = Vec::new();
        for i in r[2]..=r[3] {
            aya.push(i);
        }
        /*
        2-4, 3-8
        3-8, 2-4

        // 2,4,3,8

        .234....
        ..345678

        ..345678
        .234....

        */
        print!(
            "{:?}:{:?} | {:?}:{:?} => ",
            yay.first(),
            yay.last(),
            r[2],
            r[3]
        );
        for x in yay {
            if aya.contains(&x) {
                sum += 1;
                print!("Yes");
                println!();
                break;
            } else {
                print!("No");
                println!();
            }
        }
    }
    println!("{:?}", sum);
}
