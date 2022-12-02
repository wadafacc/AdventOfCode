/*
A: Rock
B: Paper
C: Scissors

X: Rock
Y: Paper
Z: Scissors

A - X: Draw
A - Y: Loss
A - Z: Win

B - X: Win
B - Y: Draw
B - Z: Loss

C - X: Loss
C - Y: Win
C - Z: Draw
*/

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open("./data.txt").unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let mut split: Vec<&str> = line.split(" ").collect();

        //selection points
        if split[1] == "X" {
            sum += 1;
            if split[0] == "A" {
                sum += 3;
            }
            if split[0] == "C" {
                sum += 6;
            }
        }
        if split[1] == "Y" {
            sum += 2;
            if split[0] == "B" {
                sum += 3;
            }
            if split[0] == "A" {
                sum += 6;
            }
        }
        if split[1] == "Z" {
            sum += 3;
            if split[0] == "C" {
                sum += 3;
            }
            if split[0] == "B" {
                sum += 6;
            }
        }
    }
    println!("{:?}", sum);
}
