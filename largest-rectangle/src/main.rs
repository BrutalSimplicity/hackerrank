// https://www.hackerrank.com/challenges/largest-rectangle
use std::io;
use std::str;

fn read_value<T: str::FromStr>() -> T {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read line");
  let val: T = match input.trim().parse::<T>() {
    Ok(n) => n,
    _ => panic!("Error parsing value"),
  };

  val
}

fn read_line() -> String {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read line");

  input.trim().to_string()
}

fn solve() {
  // recursively find the maximum area.
  // Base Case 0: return 0
  // Base Case 1: return the only value
  // If there are more elements, find the minimum value of those
  // elements and then calculate the area based on that minimum and the
  // number of heights. Then recurse on the bottom and upper half of the
  // set of heights. The final answer will be the maximum area found.
  // (In my head this makes sense, but if you are reading this you might be thinking
  // wtf! Checkout the hacker rank problem for clarification)
  fn find_max_area(heights: &[u32], max_area: u32) -> u32 {
    let new_max = match heights.len() {
      0 => 0,
      1 => heights.first().unwrap().to_owned(),
      _ => {
        let mut min_height = heights[0];
        let mut min_height_idx = 0;
        for (idx, h) in heights.iter().enumerate() {
          if *h < min_height {
            min_height = *h;
            min_height_idx = idx;
          }
        }
        let mut temp_max = min_height * (heights.len() as u32);
        if temp_max < max_area {
          temp_max = max_area;
        }
        let temp_max1 = find_max_area(&heights[0..min_height_idx], temp_max);
        let temp_max2 = find_max_area(&heights[min_height_idx + 1..heights.len()], temp_max);

        if temp_max1 > temp_max2 { temp_max1 } else { temp_max2 }
      }
    };

    if new_max > max_area { new_max } else { max_area }
  }

  let heights: Vec<u32> = read_line().split_whitespace()
                                     .map(|height| height.trim().parse::<u32>().unwrap())
                                     .collect();
  let max_area = find_max_area(&heights[..], 0);

  println!("{:?}", max_area);
}

fn main() {
    let _ = read_value::<i32>();
    solve();
}
