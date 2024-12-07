use std::{collections::{HashMap, HashSet}, fs::read_to_string, thread, time::Duration};

//                                  up    right   down    left
const DIRS: [(isize,isize); 4] = [(-1,0), (0,1), (1,0), (0,-1)];

fn main() {
  let str_in = read_to_string("./inputs.txt").expect("no file found");

  let mut visited: HashMap<(isize, isize), char> = HashMap::new();
  let mut loopable: HashSet<(isize, isize)> = HashSet::new();

  let (map, mut guard_coords) = get_map(str_in);
  
  let mut counter = 0;
  let mut current_dir = 0;
  loop {
    thread::sleep(Duration::from_millis(250));
    fancy_print(map.clone(), visited.clone(),loopable.clone(), guard_coords);
    let n = get_next_tile(map.clone(), guard_coords, DIRS[current_dir]);
    match n {
      Ok((coords, c)) => {
        if visited.contains_key(&guard_coords) {
          let next_tile = get_next_tile(map.clone(), guard_coords, DIRS[turn(current_dir)]).ok().expect("oops");
          let next_next = get_next_tile(map.clone(), next_tile.0, DIRS[turn(current_dir)]).ok().expect("oops");
          
          if visited.contains_key(&next_tile.0) && visited.contains_key(&next_next.0) {
            loopable.insert(coords);
          }
        }
        visited.insert(guard_coords, c);
        if c == '#' {
          current_dir = turn(current_dir);
        }
        else {
          guard_coords = coords;
        }
      },
      Err(_) => break,
    }
  }

  println!("LEN: {:?}", visited.len() + 1);
  println!("OBSTACLE COUNTER: {:?}", counter);
}

fn fancy_print(map: Vec<Vec<char>>, visited: HashMap<(isize, isize), char>, loops: HashSet<(isize, isize)>, guard: (isize, isize)) {
  print!("\x1B[2J");
  for (y, ln) in map.iter().enumerate() {
    for (x, c) in ln.iter().enumerate() {
      if (y as isize,x as isize) == guard {
        print!("\x1b[93m^\x1b[0m");
      }
      else if loops.contains(&(y as isize,x as isize)) {
        print!("\x1b[46mO\x1b[0m");
      }
      else if visited.contains_key(&(y as isize,x as isize)) {
        print!("x");
      }
      else {
        print!("{c}");
      }
    }
    println!();
  }
}

fn turn(idx: usize) -> usize {
  let i = idx+1;
  if i > 3{
    return 0
  }
  i as usize
}

fn get_next_tile(map: Vec<Vec<char>>, gc: (isize, isize), cd: (isize, isize)) -> Result<((isize, isize), char), bool> {
  let next_pos = (gc.0 + cd.0, gc.1 + cd.1);

  if (next_pos.0 >= 0 && next_pos.0 < map.len() as isize) && (next_pos.0 >= 0 && next_pos.1 < map[0].len() as isize) {
    let coords = (next_pos.0, next_pos.1);
    return Ok((coords, map[coords.0 as usize][coords.1 as usize]))
  }

  Err(false)
}

fn get_map(str_in: String) -> (Vec<Vec<char>>, (isize, isize)) {
  let mut map: Vec<Vec<char>> = Vec::new();
  let mut coords = (0 as isize,0 as isize);

  for (y,ln) in str_in.lines().enumerate() {
    let mut t: Vec<char> = Vec::new();
    for (x,c) in ln.chars().enumerate() {
      t.push(c);

      if c == '^' {
        coords = (y as isize,x as isize);
      }
    }
    map.push(t);
  }

  (map, coords)
}
