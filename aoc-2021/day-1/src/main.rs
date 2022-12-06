fn main() {
    let file: Vec<i32> = include_str!("../data.txt")
        .lines()
        .into_iter()
        .map(|i| i.parse::<i32>().unwrap())
        .collect();

    println!("{:?}", compare_signals(&file));
}

fn check_signal(input: &Vec<i32>) -> i32 {
    let mut counter = 0;

    input.into_iter().enumerate().skip(1).for_each(|(i, _s)| {
        if input[i - 1] < input[i] {
            counter += 1;
        }
    });

    counter
}

fn compare_signals(input: &Vec<i32>) -> i32 {
    let mut counter = 0;

    (0..input.len() - 1).into_iter().skip(2).for_each(|i| {
        if (input[i] + input[i - 1] + input[i - 2]) < (input[i - 1] + input[i] + input[i + 1]) {
            counter += 1;
        }
    });

    counter
}
