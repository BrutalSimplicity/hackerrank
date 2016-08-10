use std::io;
use std::str;

/// Given an array, A, of N integers, print each element in reverse order as a single line of space-separated integers.
fn main() {
  let mut input = String::new();

  // Rust has what seems to be very robust error-handling features.
  // HOwever, I'm still not complete certain how to use them.
  io::stdin().read_line(&mut input)
             .expect("Failed to read line.");

  // let's get the count
  let count: i32 = match input.trim().parse::<i32>() {
    Ok(n) => n,
    Err(err) => panic!("Error parsing integer: {}", err),
  };

  // clear the string input
  input.clear();
  io::stdin().read_line(&mut input)
             .expect("Failed to read line");

  // grab the next line and split it's input
  // this will probably be useful for future hackerrank problems
  let mut vals: Vec<i32> = Vec::new();
  for val in input.trim().split_whitespace() {
    let ival = match val.parse::<i32>() {
      Ok(n) => n,
      Err(err) => panic!("Error parsing integer: {}", err),
    };
    vals.push(ival);
  }

  // shrink down the array to size of count, just
  // in case they try to throw some extra input
  // in there for edge cases.
  let mut nums = String::new();
  vals.truncate(count as usize);
  for val in vals.iter().rev() {
    nums.push_str(&(format!("{} ", val)));
  }
  let output = nums.trim();

  println!("{}", output);
}