#[derive(Debug)]
struct Instruction {
    count: i32,
    from: usize,
    to: usize,
}

fn main() {
    let file = include_str!("../data.txt");
    let (crates, instructions) = file.split_once("\n\n").unwrap();

    let sorted_crates = stack_crates(crates);
    let sorted_instructions = translate_instructions(instructions);

    // usage: set cratemover9001 to true for the second solution, otherwise i'll spit out the first.
    let top_crates = move_crates(sorted_crates, sorted_instructions, true);
    println!("{:?}", top_crates);
}

fn move_crates(
    mut crates: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
    cratemover9001: bool,
) -> Vec<char> {
    for i in instructions {
        let mut storage: Vec<char> = Vec::new();

        for _m in 1..=i.count {
            let temp = crates[(i.from - 1)].pop().unwrap();
            storage.push(temp);
        }
        if cratemover9001 == true {
            storage.reverse();
        }
        for m in storage.into_iter().rev() {
            crates[(i.to - 1)].push(m);
        }
    }

    // pack it up
    let mut packed_crates: Vec<char> = Vec::new();
    crates.iter().for_each(|f| {
        packed_crates.push(f[f.len() - 1]);
    });

    return packed_crates;
}

fn stack_crates(crates: &str) -> Vec<Vec<char>> {
    let crates: Vec<&str> = crates.lines().into_iter().collect();

    let mut stacks = Vec::<Vec<char>>::with_capacity(9); // change this number if necessary
    for _x in 0..stacks.capacity() {
        let new: Vec<char> = Vec::new();
        stacks.push(new);
    }

    //sort crates into stacks (vectors)
    for i in crates.into_iter().rev() {
        i.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(index, c)| {
                if c.is_uppercase() {
                    stacks[index].push(c);
                }
            });
    }
    stacks
}

fn translate_instructions(instructions_unfiltered: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    for l in instructions_unfiltered.lines() {
        let line = l.replace("from", "").replace("to", "").replace("move", "");
        let array: Vec<&str> = line.split_whitespace().collect();

        instructions.push(Instruction {
            count: array[0].parse::<i32>().unwrap(),
            from: array[1].parse::<usize>().unwrap(),
            to: array[array.len() - 1].parse::<usize>().unwrap(),
        });
    }
    return instructions;
}
