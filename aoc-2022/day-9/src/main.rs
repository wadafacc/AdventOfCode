use core::panic;
use std::collections::HashSet;

#[derive(Debug, Default, Clone)]
struct Snek {
    head: (i32, i32),
    tail: (i32, i32),
    tail_hit_spaces: HashSet<(i32, i32)>,
}

fn main() {
    let input: Vec<Vec<&str>> = include_str!("../data.txt")
        .lines()
        .map(|f| f.split(" ").collect())
        .collect();

    let snek = Snek::default();

    let vals = snekify(input, snek);

    println!("{:?}", vals.tail_hit_spaces.len());
}

fn snekify(input: Vec<Vec<&str>>, mut snek: Snek) -> Snek {
    for task in input {
        let amount = task[1].parse::<i32>().unwrap();
        let direction = task[0];

        for _ in 0..amount {
            move_snek(&mut snek, direction);
            follow_snek(&mut snek);
            snek.tail_hit_spaces.insert((snek.tail.0, snek.tail.1));
        }
    }
    return snek;
}

fn move_snek(snek: &mut Snek, dir: &str) {
    match dir {
        "L" => snek.head.0 -= 1,
        "R" => snek.head.0 += 1,
        "D" => snek.head.1 -= 1,
        "U" => snek.head.1 += 1,
        _ => panic!("No input bruv"),
    }
}

/*
check where tail is supposed to go

...
.T. -> Tail should only move if its more than 1 field apart from head
..H

signum -> checks if num is positive or negative
*/
fn follow_snek(snek: &mut Snek) {
    snek.tail.0 += (snek.head.0 - snek.tail.0).signum();
    snek.tail.1 += (snek.head.1 - snek.tail.1).signum();
}
