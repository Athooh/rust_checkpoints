pub fn reverse_it(v: i32) -> String {
    if v>=0{
      format!("{}{}", v.abs().to_string().chars().rev().collect::<String>(), v.abs())
    }else{
      format!("-{}{}", v.abs().to_string().chars().rev().collect::<String>(), v.abs())
    }
  }