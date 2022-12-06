#[derive(Debug)]
struct Nav {
    direction: String,
    value: i32,
}
#[derive(Debug)]
struct Boat {
    x_cord: i32,
    depth: i32,
    aim: i32,
}

fn main() {
    let file: Vec<&str> = include_str!("../data.txt").split("\n").collect();

    let sub = Boat {
        x_cord: 0,
        depth: 0,
        aim: 0,
    };

    let res = navigate(file, sub);

    println!("{:?}", res.depth * res.x_cord);
}

fn parse_input(input: Vec<&str>) -> Vec<Nav> {
    let mut storage: Vec<Nav> = Vec::new();
    (0..input.len()).enumerate().for_each(|(i, _n)| {
        let x: Vec<&str> = input[i].split(" ").collect();
        storage.push(Nav {
            direction: String::from(x[0]),
            value: x[1].parse::<i32>().unwrap(),
        });
    });

    storage
}

fn navigate(input: Vec<&str>, mut boat: Boat) -> Boat {
    let cmds = parse_input(input);
    for i in cmds {
        if i.direction == "forward" {
            boat.x_cord += i.value;
            boat.depth += i.value * boat.aim;
        }
        if i.direction == "down" {
            boat.aim += i.value;
        }
        if i.direction == "up" {
            boat.aim -= i.value;
        }
    }

    return boat;
}
