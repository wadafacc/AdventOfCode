fn main() {
    let file: Vec<&str> = include_str!("../data.txt").lines().collect();
    let mut list: Vec<Vec<char>> = Vec::new();

    file.into_iter().for_each(|c| {
        let mut v: Vec<char> = Vec::new();
        for i in c.chars() {
            v.push(i);
        }
        list.push(v);
    });

    let gamma = get_gamma_rate(list);
    let epsylon = get_epsylon_rate(gamma);
    println!("{:?}", gamma * epsylon);
}

fn get_gamma_rate(list: Vec<Vec<char>>) -> i32 {
    let mut common: Vec<char> = Vec::new();

    (0..list[0].len()).into_iter().for_each(|c| {
        let mut count_zero = 0;
        let mut count_one = 0;
        (0..list.len()).into_iter().for_each(|r| {
            if list[r][c] == '0' {
                count_zero += 1;
            } else {
                count_one += 1;
            }
        });
        if count_one > count_zero {
            common.push('1');
        } else {
            common.push('0');
        }
    });
    let x: String = common.into_iter().collect();
    return i32::from_str_radix(&x, 2).unwrap();
}

fn get_epsylon_rate(int: i32) -> i32 {
    return int ^ 0xFFF;
}
