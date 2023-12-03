use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open("./data.txt").unwrap();
    let lines:Vec<String> = BufReader::new(file).lines().map(|l|l.expect("cant read lad")).collect();

    let mut numbers:HashMap<(u32,u32), u32> = HashMap::new();

    lines.iter().enumerate().for_each(|(y,line)| {
        for (x,c) in line.chars().enumerate() {
            if !c.is_numeric() && c != '.' {
                numbers.extend(check_surrounding(lines.clone(),x,y));
            }
        }   
    });

    for e in numbers.clone() {
        (e.0.0-1..=e.0.0+1).for_each(|num| {
            if numbers.contains_key(&(num, e.0.1)) {
                numbers.remove(&(num, e.0.1));
            }
        });
        (e.0.1-1..=e.0.1+1).for_each(|num| {
            if numbers.contains_key(&(e.0.0,num)) {
                numbers.remove(&(e.0.0,num));
            }
        });
    }

    println!("{:?}", numbers);
}

fn check_surrounding(lines: Vec<String>, x:usize, y:usize) -> HashMap<(u32,u32), u32> {
    let mut numbers:HashMap<(u32,u32), u32> = HashMap::new();
    (y-1..=y+1).for_each(|i| {
        (x-1..=x+1).for_each(|j| {
            let line:Vec<char> = lines[i].chars().collect();
            if line[j].is_numeric() {
                numbers.insert((j as u32,i as u32), get_num(line,j));

            }
        });
    });

    return numbers;
}

fn get_num(line: Vec<char>,x:usize) -> u32{
    let mut num:Vec<char> = Vec::new();

    num.push(line[x]);

    for (_i,n) in line.clone().as_slice()[..x].iter().rev().enumerate() {
        if n.is_numeric() {
            num.reverse();
            num.push(*n);
            num.reverse();
        }
        if *n == '.' {
            break;
        }
    }

    for (_i,n) in line.clone().as_slice()[x+1..].iter().enumerate() {
        if n.is_numeric() {
            num.push(*n);
        }
        if *n == '.' {
            break;
        }
    }

    let number:String = num.iter().collect();
    return number.parse::<u32>().expect("no unwrap");
}
