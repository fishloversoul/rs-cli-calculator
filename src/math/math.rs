pub fn eval(input: Vec<String>) -> Result<f32, String> {
  let mut parts: Vec<String> = input;
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
        "^" => num1.powf(num2),
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
    if parts[i] == "(" {
      let mut l = i + 1;
      while l < parts.len() {
        if parts[l] == ")" {
          break;
        } else {
          l += 1;
        }
      }
      let mut parts1 = parts[i + 1..l].to_vec();
      //println!("{:?}", parts1);
      red_ops_do(&mut parts1)?;
      let answer = parts1[0].to_string();
      parts[i] = answer;
      for _u in 0..(l - i) {
        parts.remove(i + 1);
      }
      //println!("{:?}", parts1);
      //println!("{:?}", parts);
      //println!("worked");
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
