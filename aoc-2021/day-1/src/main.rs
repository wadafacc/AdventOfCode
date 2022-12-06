fn main() {
    let file: Vec<&str> = include_str!("../data.txt").lines().collect();

    println!("{:?}", check_signal(&file));
}

fn check_signal(input: &Vec<&str>) -> i32 {
    let mut counter = 0;

    input.into_iter().enumerate().skip(1).for_each(|(i, _s)| {
        print!("{:?} | {:?} -> ", input[i - 1], input[i]);
        if input[i - 1].parse::<i32>().unwrap() < input[i].parse::<i32>().unwrap() {
            print!(" Bigger");
            counter += 1;
        }

        println!();
    });

    counter
}
