fn main() {
    let input: Vec<Vec<&str>> = include_str!("../data.txt")
        .lines()
        .map(|f| f.split(" ").collect())
        .collect();

    println!(
        "Sum: {:?}",
        exec(input, [20, 60, 100, 140, 180, 220].to_vec())
    );
}

fn render(crt: String) {
    let mut row: String = String::from("");

    crt.chars().into_iter().enumerate().for_each(|(pixel, c)| {
        row += &c.to_string();
        if pixel == 39
            || pixel == 79
            || pixel == 119
            || pixel == 159
            || pixel == 199
            || pixel == 239
        {
            println!("{:?}", row);
            row = String::from("");
        }
    });
    println!();
}

fn draw_crt(reg: i32, cycle: i32) -> char {
    if cycle == reg || cycle == reg + 1 || cycle == reg + 2 {
        return '#';
    } else {
        return '.';
    }
}

fn exec(input: Vec<Vec<&str>>, _cycle_points: Vec<i32>) {
    let mut crt: String = String::from("");
    let mut cycles = 0;
    let mut registry = 1;

    for l in input {
        for _ in 0..l.len() {
            if cycles == 40 {
                cycles = 0;
            }
            cycles += 1;
            crt += &draw_crt(registry, cycles).to_string();

            // println!(
            //     "Current Cycle: {:?} | Reg: {:?} | crt {:?}",
            //     cycles, registry, crt
            // );
        }
        command(l, &mut registry);
    }
    render(crt);
}

fn command(cmd: Vec<&str>, registry: &mut i32) {
    match cmd[0] {
        "noop" => (),
        "addx" => *registry += cmd[1].parse::<i32>().unwrap(),
        _ => panic!("input needed bruv"),
    }
}
