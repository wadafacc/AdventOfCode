use core::panic;
use std::collections::{BTreeMap};

#[derive(Default, Debug, Clone)]
struct Monke {
    id: i32,
    operation: Operation,
    test: u128,
    true_outcome: i32,
    false_outcome: i32,
    inspections: u128,
}

#[derive(Default, Debug, Clone)]
struct Operation {
    operand: char,
    last: String,
}

#[derive(Default, Debug, Clone)]
struct Item {
    monke_id:i32,
    value:u128
}

fn main() {
    let input: Vec<&str> = include_str!("../data.txt").lines().collect();
    let (mut monkes, mut inventory) = parse_input(input);
    process_business(&mut monkes, &mut inventory, 10000);
    monkes.iter().for_each(|m|println!("{:?}", m.inspections));
}



fn parse_op(op:char, i:u128,last:u128)-> u128{
    let mut new_prio = 0.0;
    match op {
        '*' => {
            new_prio = ((i * last)) as f64
        }
        '/' => {
            new_prio = ((i / last)) as f64
        }
        '+' => {
            new_prio = ((i + last)) as f64
        }
        '-' => {
            new_prio = ((i - last)) as f64
        }
        _ => panic!("need op bruv"),
    }

    // new_prio /= 3.0;
    // uncomment this for part 1
    return new_prio.floor() as u128;
}

fn parse_last(item: u128, last: String) -> u128 {
    if last == "old" {
        item
    } else {
        last.parse::<u128>().unwrap()
    }
}

fn process_business(monkes:&mut Vec<Monke>,inventory: &mut Vec<Item>, rounds:i32) {

    for _ in 0..rounds{
        for monkey in &mut *monkes {
            // iterate through all monkeys
            for item in &mut *inventory{
                if item.monke_id != monkey.id { // only go through this monke's inventory
                }else{
                    let last = parse_last(item.value, monkey.operation.last.clone());
                    let prio = parse_op(monkey.operation.operand, item.value,last);
                    item.value = prio;
                    if prio % monkey.test == 0 {
                        item.monke_id = monkey.true_outcome;
                        println!("TRUE: Thrown to: {:?}", monkey.true_outcome);                    
                    } else {
                        item.monke_id = monkey.false_outcome;
                        println!("FALSE: Thrown to: {:?}", monkey.false_outcome);                    
                    }
                    monkey.inspections += 1;
                }
            }
        }
    }

    /*
    ---- Structure ----
    1. Iterate through monkeys x times
        2. go through general inventory and do shiet
            3. Move values [!]
                4. delete value from origin
    */
    
    /*
    --- Prio mover ---
        if (prio) % test == 0 {
            monkes[true_val].inventory.push(prio);
            println!("TRUE: Thrown to: {:?}", true_val);                    
        } else {
            monkes[false_val].inventory.push(prio);
            println!("FALSE: Thrown to: {:?}", false_val);                    
        }
    */
}

/*
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
*/
fn parse_input(input: Vec<&str>) -> (Vec<Monke>, Vec<Item>) {
    let mut inventory: Vec<Item> = Vec::new();
    let mut monkes: Vec<Monke> = Vec::new();
    let mut new: Monke = Monke::default();
    let mut id = 0;

    for l in 0..input.len() {
        let line: Vec<&str> = input[l].split(" ").collect();

        match line[0].trim() {
            "Monkey" => {
                new = Monke::default();
                new.id = line.last().unwrap().replace(":", "").parse::<i32>().unwrap();
            },
            _ => ()
        } 

        if input[l].contains("Test"){
            let res: Vec<&str> = line.last().unwrap().split(" ").collect();
            new.test = res.last().unwrap().parse::<u128>().unwrap()
        }

        if input[l].contains("true"){
            let res: Vec<&str> = line.last().unwrap().split(" ").collect();
            new.true_outcome = res.last().unwrap().parse::<i32>().unwrap()
        }
        if input[l].contains("false"){
            let res: Vec<&str> = line.last().unwrap().split(" ").collect();
            new.false_outcome = res.last().unwrap().parse::<i32>().unwrap()
        }
        if input[l].contains("Starting") {
            let mut loin: Vec<&str> = input[l].split(":").collect();
            loin.remove(0);
            let numbas: Vec<&str> = loin[0].split(",").collect();
            let items:Vec<u128> = numbas
                .iter()
                .map(|f| f.trim().parse::<u128>().unwrap())
                .collect();

            for i in items {
                inventory.push(Item{monke_id:new.id, value:i});
            }
        }

        if input[l].contains("Operation") {
            let new_op = Operation {
                operand: line[line.len() - 2].parse::<char>().unwrap(),
                last: line.last().unwrap().parse::<String>().unwrap(),
            };
            new.operation = new_op;
        }

        // insert new monkey
        if input[l] == "" || l == input.len() - 1 {
            monkes.push(new.to_owned());
        }
    }

    return (monkes, inventory);
}

