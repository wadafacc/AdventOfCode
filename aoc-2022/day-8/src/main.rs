fn main() {
    let file: &str = include_str!("../data.txt");
    let chars: Vec<Vec<u32>> = file
        .lines()
        .map(|l| l.chars().map(|t| t.to_digit(10).unwrap()).collect())
        .collect();
    println!("{:?}", chars); // wadafak

    let mut sum = 0;

    // add edges cuz always visible
    sum += (chars.len() + chars[0].len() - 2) * 2;

    (0..chars[0].len() - 1).into_iter().skip(1).for_each(|x| {
        (0..chars.len() - 1)
            .clone()
            .into_iter()
            .skip(1)
            .for_each(|y| {
                let rows = &chars[x][0..y];
                let columns = chars
                    .iter()
                    .map(|s| s.iter().nth(y).unwrap())
                    .collect::<Vec<_>>();

                // TODO: Fix this, maybe do it with:
                /*
                - Get index of highest tree
                - check if current index of current tree is either after or before it
                - yea didn't think this through really did i
                */

                print!(" Row: {:?} | Current Char: {:?}", rows, chars[x][y]); // wadafak
                if (columns.iter().max().unwrap() > &&chars[x][y])
                    || (rows.iter().max().unwrap() > &&chars[x][y])
                {
                    print!(" bigger");
                    sum += 1;
                }
                println!();
            });
    });
    println!();

    println!("Visible Trees: {:?}", sum);
}
