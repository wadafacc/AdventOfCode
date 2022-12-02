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
        let mut enemy = split[0];
        //selection points
        if split[1] == "X" {
            //lose
            if enemy == "A" {
                sum += 3;
            }
            if enemy == "B" {
                sum += 1;
            }
            if enemy == "C" {
                sum += 2;
            }
        }
        if split[1] == "Y" {
            //draw
            if enemy == "A" {
                sum += 4;
            }
            if enemy == "B" {
                sum += 5;
            }
            if enemy == "C" {
                sum += 6;
            }
        }
        if split[1] == "Z" {
            //win
            if enemy == "A" {
                sum += 8;
            }
            if enemy == "B" {
                sum += 9;
            }
            if enemy == "C" {
                sum += 7;
            }
        }
    }
    println!("{:?}", sum);
}
