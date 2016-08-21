// https://www.hackerrank.com/challenges/counter-game
use std::io;
use std::str;

enum Players {
  Richard,
  Louise
}

fn is_power_of_two(val: u64) -> bool {
  val & (val - 1) == 0
}

fn get_msb(val: u64) -> u64 {
  let mut msb = 0;
  let mut n = val;
  while n > 0 {
    msb += 1;
    n >>= 1;
  }
  msb
}

fn switch_turn(player: Players) -> Players {
  match player {
    Players::Richard => Players::Louise,
    Players::Louise => Players::Richard,
  }
}

fn play_game(n: u64, player: Players) -> Players {
  if n == 1 {
    return switch_turn(player);
  }
  play_game_recurse(n, player)
}

fn play_game_recurse(n: u64, player: Players) -> Players {
  if n == 1 {
    return switch_turn(player);
  }

  if is_power_of_two(n) {
    return play_game_recurse(n - (n >> 1), switch_turn(player));
  }
  else {
    return play_game_recurse(n - (1 << get_msb(n) - 1), switch_turn(player));
  }
}

fn read_value<T: str::FromStr>() -> T {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read line");
  let val: T = match T::from_str(input.trim()) {
    Ok(n) => n,
    _ => panic!("Error parsing value"),
  };

  val
}

fn main() {

  let count = read_value::<i32>();

  for _ in 0..count {
    let n = read_value::<u64>();
    match play_game(n, Players::Louise) {
      Players::Louise => println!("Louise"),
      _ => println!("Richard")
    }
  }
}
