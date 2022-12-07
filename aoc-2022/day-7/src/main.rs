#[derive(Debug, Clone)]
struct Dir {
    id: usize,
    name: String,
    subdirs: Vec<String>,
    files: Vec<i32>,
    size: i32,
}

fn main() {
    let file: Vec<&str> = include_str!("../data.txt").lines().collect();

    let res = sum_dirs(sort_dirs(file), 100000);

    println!("Sum: {:?}", res);
}

fn sum_dirs(dirs: Vec<Dir>, max_size: i32) -> i32 {
    let mut sum = 0;
    let copy = dirs.to_owned();
    for d in dirs {
        let mut val = 0;
        println!("name: {:?}", d);
        for dir in d.subdirs {
            val = copy.iter().find(|n| n.name == dir).unwrap().size;
            println!("{:?}", val);
        }
        println!("name: {:?}, size: {:?}", d.name, d.size);
        val += d.size;

        if val < max_size {
            sum += val;
        }
        println!("Sum: {:?}", sum);
    }

    sum
}

fn sort_dirs(file: Vec<&str>) -> Vec<Dir> {
    let mut dirs: Vec<Dir> = Vec::new();

    (0..file.len()).into_iter().for_each(|i| {
        // check if line starts with 'cd'
        if file[i].contains(&"$ cd") && !file[i].contains("$ cd ..") {
            // println!("{:?}", file[i]);
            let mut new_dir = Dir {
                id: i,
                name: String::from(file[i].split(" ").last().unwrap()),
                subdirs: Vec::new(),
                files: Vec::new(),
                size: 0,
            };

            for x in (i + 1)..file.len() {
                // println!("{:?}", new_dir);
                let c: Vec<char> = file[x].chars().collect();
                let l: Vec<&str> = file[x].split(" ").collect();
                if file[x].contains("dir") {
                    new_dir.subdirs.push(l[1].to_string());
                } else if c[1].is_numeric() {
                    new_dir.files.push(l[0].parse::<i32>().unwrap());
                }
                // check for new dir / EOF
                if file[x].contains("$ cd") || x == file.len() - 1 {
                    new_dir.size = new_dir.files.iter().sum();
                    dirs.push(new_dir);

                    break;
                }
            }
        }
    });

    dirs
}
