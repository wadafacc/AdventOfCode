use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug, Default)]
struct Bag {
    red: i32,
    green: i32,
    blue: i32
}


fn main() {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open("./data.txt").unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let (_id, vals) = line.split_once(":").unwrap();

        let mut biggest = Bag {..Default::default()};
        for attempt in vals.split(";") {
            check_cubes(attempt.to_string(), &mut biggest);
        }

        sum += biggest.red * biggest.green * biggest.blue;
    }


    println!("{}",sum);
}

fn check_cubes(cubes: String, highest: &mut Bag){


    for c in cubes.split(",") {
        let (a, color) = c.trim().split_once(" ").unwrap();
        let amnt = a.parse::<i32>().unwrap();

        match color {
            "red" => {
                highest.red = if amnt > highest.red { amnt } else { highest.red };
            },
            "green" => {
                highest.green = if amnt > highest.green { amnt } else { highest.green };
            },
            "blue" => {
                highest.blue = if amnt > highest.blue { amnt } else { highest.blue };
            },
            _ => panic!()
        }
    }
}