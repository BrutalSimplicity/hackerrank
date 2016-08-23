// https://www.hackerrank.com/challenges/balanced-brackets
use std::io;
use std::str;


fn read_value<T: str::FromStr>() -> T {
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read line");
  let val: T = match T::from_str(input.trim()) {
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


fn get_closing_bracket(c: &char) -> char {
	match *c {
		'[' => ']',
		'{' => '}',
		'(' => ')',
		_ => '\0',
	}
}



fn main() {
	let num_tests = read_value::<u16>();

	for _ in 0..num_tests {
		let line = read_line();
		let mut bracket_stack: Vec<char> = Vec::new();

		// read each character in the string.
		// The only possible characters are: [ ] { } ( )
		// This means that we can just push the opening bracket onto the stack,
		// and if we have a matching closing bracket then we pop it off the stack
		// if the stack is not empty after we are done, then the string is unbalanced,
		// otherwise it is balanced
		for bracket in line.chars() {
			if !bracket_stack.is_empty() {
				if get_closing_bracket(bracket_stack.last().unwrap()) == bracket {
					bracket_stack.pop();
					continue;
				}
			}
	    bracket_stack.push(bracket);
		}
		if !bracket_stack.is_empty() {
			println!("NO");
		}
		else {
		    println!("YES");
		}
	}
}
