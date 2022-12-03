use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Index;

fn main() {
    //init alphabet and use index pos as value
    let alphabet: Vec<char> = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect())
        .unwrap()
        .chars()
        .collect();

    let file = File::open("./data.txt").unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        //half the string
        let mut x: Vec<&str> = line[(line.len() / 2)..].split("").collect();
        let mut y: Vec<&str> = line[..(line.len() / 2)].split("").collect();
        x.remove(x.len() - 1 as usize);
        y.remove(y.len() - 1 as usize);
        x.remove(0);
        y.remove(0);

        println!("{:?}", x);
        println!("{:?}", y);

        for item in &x {
            if y.contains(item) {
                let i = item.chars().next().unwrap();
                let index = alphabet.iter().position(|&r| r == i).unwrap();
                sum += index + 1;
                break;
            }
        }
    }
    println!("{:?}", sum);
}
