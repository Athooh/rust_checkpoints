pub fn reverse_it(v: i32) -> String {
    let v1 = v.to_string();
    let v2: String = v.abs().to_string().chars().rev().collect();

    if v < 0 {
        format!("-{}{}", &v2, v1[1..].to_string())
    } else {
        format!("{}{}", &v2, &v1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
