use std::io::{self, Write};

use crate::math::math::eval;
pub mod math;

pub fn cal() {
  let mut last_ans: f32 = 0.0;
  loop {
    print!("equation: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("something bad 1");

    let trimed = input.trim().to_lowercase();
    if trimed == "esc" || trimed == "exit" || trimed == "" {
      break;
    }

    let token: Vec<String> = input
      .trim()
      .split_whitespace()
      .map(|t| {
        if t.eq_ignore_ascii_case("ans") {
          last_ans.to_string()
        } else {
          t.to_string()
        }
      })
      .collect();

    let rebuilt = token.join(" ");

    match eval(&rebuilt.trim()) {
      Ok(value) => {
        println!("{}", value);
        last_ans = value
      }
      Err(value) => println!("Err {}", value),
    }
  }
}
