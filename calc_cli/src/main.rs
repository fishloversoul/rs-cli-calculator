use calc_core::eval;
use std::io::{self, Write};

fn main() {
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

    match eval(cinput) {
      Ok(value) => {
        println!("{}", value);
        last_ans = value
      }
      Err(value) => println!("Err {}", value),
    }
  }
}
