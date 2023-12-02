use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy)]
struct Bag {
    red: i32,
    green: i32,
    blue: i32
}


fn main() {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open("./data.txt").unwrap();
    let reader = BufReader::new(file);

    let mut valid_games :Vec<i32> = Vec::new();


    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14
    };

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let (id, vals) = line.split_once(":").unwrap();

        let mut possible = true;
        for attempt in vals.split(";") {

            if !check_cubes(attempt.to_string(), bag) {
                possible = false;
            }
        }

        if possible {
            valid_games.push(id.split_once(" ").unwrap().1.parse().unwrap());
        }

    }

    let mut sum = 0;
    for id in valid_games {
        sum += id;
    }
    println!("{}",sum);
}

fn check_cubes(cubes: String, bag:Bag) -> bool {
    let mut bag = bag;
    
    for c in cubes.split(",") {
        let (a, color) = c.trim().split_once(" ").unwrap();
        let amnt = a.parse::<i32>().unwrap();

        match color {
            "red" => bag.red -= amnt,
            "green" => bag.green -= amnt,
            "blue" => bag.blue -= amnt,
            _ => panic!()
        }

        if (bag.red < 0) | (bag.green < 0) | (bag.blue < 0) {
            return false;
        }
    }
    return true
}