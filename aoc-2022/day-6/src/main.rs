use std::collections::HashSet;

fn main() {
    let file = include_str!("../data.txt");

    println!("Index: {:?}", find_markers(file, 14));
}

fn find_markers(file: &str, marker_size: usize) -> Option<usize> {
    let res = (0..file.len())
        .find(|&i| {
            file[i..i + marker_size]
                .chars()
                .collect::<HashSet<_>>()
                .len()
                == marker_size
        })
        .map(|ind| (ind + marker_size));
    res
}

/*
--- old somewhat working solution ---

fn find_markers(file: Chars, marker_size: usize) {
    let mut read_chars: Vec<char> = Vec::new();
    let mut counter = 0;
    file.enumerate().for_each(|(index, c)| {
        read_chars.push(c);

        if read_chars.len() >= marker_size {
            let mut current_set: Vec<char> = vec![' '; marker_size];
            current_set.copy_from_slice(&&read_chars[(read_chars.len() - marker_size)..]);

            let string: String = current_set.iter().collect();
            for i in current_set {
                if string.matches(i).count() == 1 {
                    counter += 1;
                } else {
                    counter = 0;
                }
            }
            if counter == marker_size {
                println!("index: {:?} | slice: {:?}", index - 1, string);
            }
        }
    })
}

*/
