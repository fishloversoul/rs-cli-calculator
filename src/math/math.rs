pub fn eval(input: &str) -> Result<f32, String> {
  let mut parts: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
  reducs_ops(&mut parts, "*", "/")?;
  reducs_ops(&mut parts, "+", "-")?;
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
        .map_err(|_| "not a number".to_string())?;
      let num2 = parts[i + 1]
        .parse::<f32>()
        .map_err(|_| "not a number".to_string())?;
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
