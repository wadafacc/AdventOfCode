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
    crt.chars().into_iter().enumerate().for_each(|(pixel, c)| {
        print!("{:?}", c);
        if pixel == 39
            || pixel == 79
            || pixel == 119
            || pixel == 159
            || pixel == 199
            || pixel == 239
        {
            println!();
        }
    });
    println!();
}

fn draw_crt(reg: i32, cycle: i32) -> char {
    let crt_line_len = [40, 80, 120, 160, 200, 240];
    let mut c = 0;
    if crt_line_len.contains(&c) {
        c = 0;
    } else {
    }
    if c == reg || c == reg + 1 || c == reg + 2 {
        return '#';
    } else {
        return '.';
    }
}

fn exec(input: Vec<Vec<&str>>, cycle_points: Vec<i32>) -> i32 {
    let mut crt: String = String::from("");
    let mut cycles = 0;
    let mut registry = 1;
    let mut sum = 0;

    for l in input {
        for _ in 0..l.len() {
            cycles += 1;
            crt += &draw_crt(registry, cycles).to_string();
            println!(
                "Current Cycle: {:?} | Reg: {:?} | crt {:?}",
                cycles, registry, crt
            );

            // part 1
            if cycle_points.contains(&cycles) {
                sum += cycles * registry;
            }
        }
        command(l, &mut registry);
    }
    // render(crt);
    sum
}

fn command(cmd: Vec<&str>, registry: &mut i32) {
    match cmd[0] {
        "noop" => (),
        "addx" => *registry += cmd[1].parse::<i32>().unwrap(),
        _ => panic!("input needed bruv"),
    }
}
