use std::fs::read_to_string;

fn main() {
  let str = read_to_string("./inputs.txt").expect("no input");
  let input: Vec<u32> = str.chars().map(|i| i.to_digit(10).unwrap()).collect();

  let sum: u32 = input.iter().sum();
  let mut drive: Vec<Option<u32>> = Vec::with_capacity(sum as usize);


  // add to "drive"
  let mut c = 0;
  for (i,x) in input.iter().enumerate() {
    if i % 2 != 0 {
      drive.extend(vec![None;*x as usize].iter());
    }
    else {
      drive.extend(vec![Some(c as u32);*x as usize].iter());
      c += 1;
    }
  }

  for i in (0..drive.len()).rev() {
    if drive[i].is_some() {
      let ffpi = drive[0..i-1].iter().position(|e| e.is_none());  // First Free Place Index™️
      if ffpi.is_none() {
        break;
      }

      let temp = drive[i];
      drive[i] = drive[ffpi.unwrap()];
      drive[ffpi.unwrap()] = temp;

    }
  }


  let mut sum: u128 = 0;
  for (i,e) in drive.iter().enumerate() {
    if let Some(o) = e {
      sum += (i as u128) * (*o as u128);
    }
  }

  println!("{:?}", sum);
}
