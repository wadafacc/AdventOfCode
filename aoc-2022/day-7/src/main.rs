use std::collections::{HashMap, HashSet};

fn main() {
    let file: Vec<&str> = include_str!("../data.txt").lines().collect();

    // let res = sum_dirs(, 100000);

    println!("Sum: {:?}", sort_dirs(file));
}

// path concept very smart, copied from some guy on reddit
fn cd(i: &str, cur: &mut Vec<String>) {
    match i {
        ".." => {
            cur.pop();
        }
        "/" => {
            cur.clear();
        }
        _ => {
            // wildcard -> dir name
            cur.push(i.to_string());
        }
    };
}

// TODO: Recursion bs | copied from some other dude on reddit, cuz got no fucks left to give
fn dir_size(
    from: &str,
    dirs: &HashMap<String, HashSet<(&str, usize)>>,
    counted: &mut HashSet<String>,
) -> usize {
    let mut s = dirs
        .get(from)
        .unwrap()
        .iter()
        .filter_map(|&(f, size)| {
            let full_path = [from, f].join("");
            let n = (!counted.contains(&full_path)).then(|| size);
            counted.insert(full_path);
            n
        })
        .sum();

    dirs.keys()
        .filter(|l| l.starts_with(from))
        .filter(|&p| p != from)
        .for_each(|d| s += dir_size(d, dirs, counted));
    s
}

fn sort_dirs(file: Vec<&str>) -> (usize, usize) {
    let mut path: Vec<String> = vec![];
    let mut dirs: HashMap<String, HashSet<(&str, usize)>> = HashMap::new();

    file.into_iter().for_each(|line| {
        if line.starts_with("$") {
            let s: Vec<_> = line.split_whitespace().collect();
            // sort out the cmds and change path
            match s[1] {
                "ls" => (),
                "cd" => cd(s[2], &mut path),
                _ => panic!(),
            }
        } else {
            let current_dir = path.join("/");
            //insert dir into list
            if !dirs.contains_key(&current_dir) {
                dirs.insert(current_dir.clone(), HashSet::new());
                // some advanced bullshittery right here | GOTTA LEARN HOW FUCKIN HASHMAPS WORK
            }

            // process files
            if !line.starts_with("dir") {
                let (size, name) = line.split_once(" ").unwrap();
                let size = size.parse().unwrap();
                dirs.get_mut(&current_dir).unwrap().insert((name, size));
            }
        }
    });
    let sizes: Vec<_> = dirs
        .keys()
        .map(|k| dir_size(k, &dirs, &mut HashSet::new()))
        .collect();

    let delta = 30_000_000 + sizes.iter().max().unwrap() - 70_000_000;
    (
        sizes.iter().filter(|&&n| n <= 100_000).sum(),
        *sizes.iter().filter(|&&n| n >= delta).min().unwrap(),
    )
}
