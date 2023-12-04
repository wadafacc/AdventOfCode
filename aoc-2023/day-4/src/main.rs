use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {

    let file = File::open("./data.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l|l.expect("cant read lad")).collect();
    
    let mut cards:Vec<(i32, usize)> = Vec::new();

    lines.iter().enumerate().for_each(|(_i,line)| {
        let (card_nums, winning_nums) = 
        line.split_once(":")
        .unwrap().1.split_once("|").unwrap();
        
        println!("CARD: {} | {}", card_nums, winning_nums);

        let mut nums: Vec<&str> = card_nums.split(" ").collect();
        nums.retain(|&x|x != "");

        let mut to_check: Vec<&str> = winning_nums.split(" ").collect();
        to_check.retain(|&x|x != "");

        to_check.retain(|&i| nums.contains(&i));

        println!("WINNING NUMBERS: {:?}", to_check);

        if to_check.len() > 0 {
        }
        cards.push((1,to_check.len()));
    });

    for i in 0..cards.len() {
        for _ in 0..cards[i].0 {
            for j in 1..=cards[i].1 {
                cards[i+j].0 += 1;
            }
        }
    }

    for x in cards.clone() {
        println!("CARD: {:?}",x);
    }

    let total:i32 = cards.iter().map(|x|x.0).sum();

    println!("{:?}", total);
}