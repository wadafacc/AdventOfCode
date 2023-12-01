use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("./data.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let first_re = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let last_re = Regex::new(r"\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();

    let mut sum = 0;
    for line in lines {
        let input = line.unwrap();
        let reversed: String = input.chars().rev().collect();
        let first_digit = get_number(first_re.find(&input).unwrap().as_str(), false);
        let last_digit = get_number(last_re.find(&reversed).unwrap().as_str(), true);
        let number: u64 = format!("{}{}", first_digit, last_digit).parse().unwrap();
        sum += number;
    }

    println!("{sum}");
}

fn get_number(input: &str, reverse: bool) -> &str {
    let numstr: String = if reverse {
        input.chars().rev().collect()
    } else {
        input.to_string()
    };
    match numstr.as_str() {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => input,
    }
}