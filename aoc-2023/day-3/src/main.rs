use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open("./data.txt").unwrap();
    let lines:Vec<String> = add_padding(BufReader::new(file).lines().map(|l|l.expect("cant read lad")).collect());


    println!("{:?}", lines);

    let mut numbers:Vec<String> = Vec::new();
    let mut temp_num = "".to_string();

    lines.iter().enumerate().for_each(|(y,line)| {
        for (x,c) in line.chars().enumerate() {
            
            if c.is_numeric(){
                temp_num += &c.to_string();
            }

            if c == '.' && temp_num != "" && check_surrounding(lines.clone(),x,y) {
                numbers.push(temp_num.clone());
                temp_num = "".to_string();
            }
        }   
    });

}

fn check_surrounding(lines: Vec<String>, x:usize, y:usize) -> bool {
    let mut out: bool = false;

    (y-1..=(x+1).max(lines.len()-1)).for_each(|i| {
        (x-1..=(x+1).max(lines[0].len()-1)).for_each(|j| {
            let line:Vec<char> = lines[i].chars().collect();
            if !line[j].is_numeric() && line[j] != '.' {
                out = true;
            }
        });
    });
    return out;
}

fn add_padding(input: Vec<String>) -> Vec<String> {
    let mut with_padding:Vec<String> = Vec::new();

    with_padding.push(".".repeat(input[0].len()));
    for row in input.clone() {
        let temp = ".".to_string() + &row + ".";
        with_padding.push(temp);
    }
    with_padding.push(".".repeat(input[0].len()));

    return with_padding;
}