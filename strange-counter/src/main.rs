// https://www.hackerrank.com/contests/master/challenges/strange-code?h_r=internal-search

use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input)
             .expect("Failed to read line.");

  let time: u64 = match input.trim().parse::<u64>() {
    Ok(n) => n,
    Err(err) => panic!("Error parsing integer: {}", err),
  };

  println!("{:?}", strange_count(time));
}

// let's try using a function to return the count
// the pattern is that each interval of the count
// increases by 3*2^n, so we just sum these values
// until we reach the time, then subtract the time
// from that count to find out where we are in the
// interval
fn strange_count(t: u64) -> u64 {
  let mut count = 0;
  for i in 0.. {
    let interval = 3 * 2u64.pow(i);
    count += interval;
    if count >= t {
      break;
    }
  }
  count - (t - 1)
}
