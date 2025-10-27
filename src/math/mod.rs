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

    let cinput = input.replace("ans", (last_ans.to_string()).as_str());
    let token: Vec<String> = tokenizer(cinput.as_str());

    match eval(token) {
      Ok(value) => {
        println!("{}", value);
        last_ans = value
      }
      Err(value) => println!("Err {}", value),
    }
  }
}

fn tokenizer(input: &str) -> Vec<String> {
  let cinput = input
    .trim()
    .chars()
    .filter(|c| !c.is_whitespace())
    .collect::<String>();
  let mut curent: String = String::new();
  let mut output: Vec<String> = vec![];
  let mut is_number = true;
  for c in cinput.chars() {
    if c.is_digit(10) || c == '.' {
      curent.push(c);
      is_number = false;
    } else if "*/-+^()".contains(c) {
      if "-".contains(c) && is_number == true {
        curent.push(c);
        continue;
      }
      if !curent.is_empty() {
        output.push(std::mem::take(&mut curent));
      }
      output.push(c.to_string());
    }
  }
  if !curent.is_empty() {
    output.push(std::mem::take(&mut curent));
  }
  output
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn ts_tokenizer() {
    let output = tokenizer("-10+4-5");
    println!("{:?}", output);

    assert_eq!(output, vec!["-10", "+", "4", "-", "5"]);
  }
}
