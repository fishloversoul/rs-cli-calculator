#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

pub fn eval(input: String) -> Result<f32, String> {
  let mut parts: Vec<String> = tokenizer(&input);
  perent_t(&mut parts)?;
  red_ops_do(&mut parts)?;
  let result = parts[0]
    .parse::<f32>()
    .map_err(|_| "final number invalid".to_string())?;
  Ok(result)
}

fn reducs_ops(parts: &mut Vec<String>, symbol: &str, symbol1: &str) -> Result<(), String> {
  let mut i = 0;
  while i < parts.len() {
    if parts[i] == symbol || parts[i] == symbol1 {
      if i == 0 || i + 1 >= parts.len() {
        return Err("operator missing operand".to_string());
      }
      let num1 = parts[i - 1]
        .parse::<f32>()
        .map_err(|_| "not a number 1 ".to_string())?;
      let num2 = parts[i + 1]
        .parse::<f32>()
        .map_err(|_| "not a number 2 ".to_string())?;
      let result = match parts[i].as_str() {
        "*" => num1 * num2,
        "/" => {
          if num2 == 0.0 {
            return Err("divide by zero".to_string());
          }
          num1 / num2
        }
        "+" => num1 + num2,
        "-" => num1 - num2,
        "^" => powf(num1, num2 as i32),
        _ => return Err("unknown operator".to_string()),
      };
      parts[i - 1] = result.to_string();
      parts.remove(i);
      parts.remove(i);
    } else {
      i += 1;
    }
  }
  Ok(())
}

fn perent_t(parts: &mut Vec<String>) -> Result<(), String> {
  let mut i = 0;
  while i < parts.len() {
    let mut depth = 0;
    let start = i;
    let mut end = i + 1;
    if parts[i] == "(" {
      depth += 1;
      while !(depth == 0) {
        if parts[end] == "(" {
          depth += 1;
        } else if parts[end] == ")" {
          depth -= 1;
        }
        if depth == 0 {
          break;
        }
        end += 1;
      }
      let mut inner_parts: Vec<String> = parts[start + 1..end].to_vec();
      perent_t(&mut inner_parts)?;
      red_ops_do(&mut inner_parts)?;
      parts[i] = inner_parts[0].to_string();
      for _ in 0..(end - start) {
        parts.remove(i + 1);
      }
    } else {
      i += 1;
    }
  }
  Ok(())
}

fn red_ops_do(mut parts: &mut Vec<String>) -> Result<(), String> {
  reducs_ops(&mut parts, "^", "**")?;
  reducs_ops(&mut parts, "*", "/")?;
  reducs_ops(&mut parts, "+", "-")?;
  Ok(())
}

fn tokenizer(input: &str) -> Vec<String> {
  let cinput = input
    .trim()
    .chars()
    .filter(|c| !c.is_whitespace())
    .collect::<String>();
  let mut curent: String = String::new();
  let mut output: Vec<String> = Vec::new();
  let mut is_number = true;
  for c in cinput.chars() {
    if c.is_digit(10) || c == '.' {
      curent.push(c);
      is_number = false;
    } else if "*/-+^()".contains(c) {
      if c == '-' && is_number == true {
        curent.push(c);
        continue;
      }
      if !curent.is_empty() {
        output.push(core::mem::take(&mut curent));
      }
      if c != ')' {
        is_number = true;
      }
      output.push(c.to_string());
    }
  }
  if !curent.is_empty() {
    output.push(core::mem::take(&mut curent));
  }
  output
}

fn powf(base: f32, exp: i32) -> f32 {
  if exp == 0 {
    return 1.0;
  }

  let mut result = 1.0;
  let mut times = exp;
  let b = base;

  let invert = times < 0;
  if invert {
    times = -times;
  }

  while times > 0 {
    result *= b;
    times -= 1;
  }

  if invert { 1.0 / result } else { result }
}
