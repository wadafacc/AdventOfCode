use std::collections::{BTreeMap, HashMap};

#[derive(Default, Debug, Clone)]
struct Monke {
    id: i32,
    inventory: Vec<i32>,
    operation: Operation,
    test: i32,
    true_outcome: i32,
    false_outcome: i32,
    inspect_count: i32,
}

#[derive(Default, Debug, Clone)]
struct Operation {
    operand: char,
    last: String,
}

fn main() {
    let input: Vec<&str> = include_str!("../data.txt").lines().collect();
    let monkes: BTreeMap<i32, Monke> = monke_do_monke_bisnis(parse(input));

    for m in monkes.values() {
        println!("{:?}", m);
    }
}

fn init_monke_bisnis(mut monkes: BTreeMap<i32, Monke>, trade_amount: i32) -> BTreeMap<i32, Monke> {
    for i in 0..trade_amount {}
    monkes
}

fn parse_last(item: i32, last: String) -> i32 {
    if last == "old" {
        item
    } else {
        last.parse::<i32>().unwrap()
    }
}

fn monke_do_monke_bisnis(mut monkes: BTreeMap<i32, Monke>) -> BTreeMap<i32, Monke> {
    for monke in monkes.to_owned() {
        println!("Monkey: {:?}", monke.0);
        println!("Inventory length: {:?}", monke.1.inventory.len());
        let mut new_prio: f32 = 0.0;
        for i in monke.1.inventory {
            println!("Inspected item: {:?}", i);

            let last = parse_last(i, monke.1.operation.last.to_owned());
            match monke.1.operation.operand {
                '*' => {
                    new_prio = ((i * last)) as f32
                }
                '/' => {
                    new_prio = ((i / last)) as f32
                }
                '+' => {
                    new_prio = ((i + last)) as f32
                }
                '-' => {
                    new_prio = ((i - last)) as f32
                }
                _ => panic!("need op bruv"),
            }
            println!("New worry level of item: {:?}", new_prio);
            
            new_prio /= 3.0;
            new_prio = new_prio.floor();
            println!("New worry level of item / 3: {:?}", new_prio);
            

            if (new_prio as i32) % monke.1.test == 0 {
                monkes.entry(monke.1.true_outcome).and_modify(|monk| {
                    monk.to_owned().inventory.push(new_prio as i32);     
                    println!("TRUE: Thrown to: {:?}", monk.id);                    
                    println!("Inventory of {:?}: {:?}", monk.id, monk.inventory);
                });
            } else {
                monkes.entry(monke.1.false_outcome).and_modify(|monk| {
                    monk.inventory.push(new_prio as i32);     
                    println!("FALSE: Thrown to: {:?}", monk.id);
                    println!("Inventory of {:?}: {:?}", monk.id, monk.inventory);
                });
            }
            //remove item from original monke
            monkes
                .entry(monke.1.id)
                .and_modify(|monk| {
                    monk.inventory.remove(0);
                    monk.inspect_count += 1;
                });
        }
    }
    return monkes;
}

/*
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
*/
fn parse(input: Vec<&str>) -> BTreeMap<i32, Monke> {
    let mut monkes: BTreeMap<i32, Monke> = BTreeMap::new();
    let mut new: Monke = Monke::default();
    let mut id = 0;

    for l in 0..input.len() {
        let line: Vec<&str> = input[l].split(" ").collect();
        if input[l].starts_with("Monkey") {
            new = Monke::default(); // init new monke
            let x = line.last().unwrap().replace(":", "");
            id = x.parse::<i32>().unwrap(); // set id
            new.id = id;
        }
        if input[l].contains("Starting") {
            let mut loin: Vec<&str> = input[l].split(":").collect();
            loin.remove(0);
            let numbas: Vec<&str> = loin[0].split(",").collect();
            new.inventory = numbas
                .iter()
                .map(|f| f.trim().parse::<i32>().unwrap())
                .collect();
        }

        if input[l].contains("Operation") {
            let new_op = Operation {
                operand: line[line.len() - 2].parse::<char>().unwrap(),
                last: line.last().unwrap().parse::<String>().unwrap(),
            };
            new.operation = new_op;
        }

        if input[l].contains("Test") {
            let res: Vec<&str> = line.last().unwrap().split(" ").collect();
            new.test = res.last().unwrap().parse::<i32>().unwrap();
        }
        if input[l].contains("true") {
            let res: Vec<&str> = line.last().unwrap().split(" ").collect();
            new.true_outcome = res.last().unwrap().parse::<i32>().unwrap();
        }
        if input[l].contains("false") {
            let res: Vec<&str> = line.last().unwrap().split(" ").collect();
            new.false_outcome = res.last().unwrap().parse::<i32>().unwrap();
        }
        // insert new monke
        if input[l] == "" || l == input.len() - 1 {
            monkes.insert(id, new.clone());
        }
    }

    return monkes;
}
