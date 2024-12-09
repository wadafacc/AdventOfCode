use std::{fs::read_to_string, time::{Duration, Instant}};

fn main() {
  let str = read_to_string("./inputs.txt").expect("no input");
  let input: Vec<u32> = str.chars().map(|i| i.to_digit(10).unwrap()).collect();

  let mut drive: Vec<String> = Vec::with_capacity(input.len());
  let mut sum: u128 = 0;


  // add to "drive"
  let mut c = 0;
  for (i,x) in input.iter().enumerate() {
    if i % 2 != 0 {
      drive.push(".".repeat(*x as usize));
    }
    else {
      drive.push(format!("{c}").repeat(*x as usize));
      c += 1;
    }
  }

  drive.retain(|c| c != "");


  let timer = Instant::now();
  for i in (0..drive.len()).rev() {
    println!("\x1B[2J");
    println!("PROGRESS: {}/{}. TIME ELAPSED: {:2?}", i, drive.len(), timer.elapsed());
    if !drive[i].contains(".") {
      let ffpi = drive[0..i].iter().enumerate().position(|(j,e)| e.contains(".") && (e.len() >= drive[i].len()));  // first free place™️
      
      if ffpi.is_some() {
        let ffpi = ffpi.unwrap();
        let diff = drive[ffpi].len() - drive[i].len();
        
        let temp = drive[i].clone();
        drive[i] = ".".repeat(drive[i].len());
        drive[ffpi] = temp;
  
        if diff > 0 {
          drive.insert(ffpi + 1, ".".repeat(diff));
        }
        
        drive = merge(drive.join(""));
      }
    }
  }

  let str = drive.join("");
  for (i,e) in str.chars().enumerate() {
    if e != '.' {
      sum += (i as u128) * (e.to_digit(10).unwrap() as u128);
    }
  }


  println!("{:?}", drive);
  println!("SUM: {:?}", sum);
}


fn merge(v: String) -> Vec<String> {
  let mut out = Vec::new();
  let v:Vec<char> = v.chars().collect();

  let mut temp: Vec<char> = Vec::new();

  for c in v {
    if (temp.contains(&c)) |( temp.len() == 0) {
      temp.push(c);
    }
    else {
      out.push(temp.iter().collect());
      temp.clear();
      temp.push(c);
    }
  }

  if temp.len() > 0 {
    out.push(temp.iter().collect());
  }
  
  out
}