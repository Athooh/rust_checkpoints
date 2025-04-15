pub fn negative_spell(n: i64) -> String {
    if n >= 0 {
        return "error: positive number".to_string();
    }

    let abs_n = n.abs();
    format!("minus {}", number_to_words(abs_n))
}

fn number_to_words(n: i64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let below_20 = ["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
                    "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", 
                    "eighteen", "nineteen"];
    let tens = ["", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
    
    let mut result = String::new();

    if n >= 1_000_000 {
        result.push_str(&number_to_words(n / 1_000_000));
        result.push_str(" million ");
        if n % 1_000_000 != 0 {
            result.push_str(&number_to_words(n % 1_000_000));
        }
    } else if n >= 1_000 {
        result.push_str(&number_to_words(n / 1_000));
        result.push_str(" thousand ");
        if n % 1_000 != 0 {
            result.push_str(&number_to_words(n % 1_000));
        }
    } else if n >= 100 {
        result.push_str(&below_20[(n / 100) as usize]);
        result.push_str(" hundred ");
        if n % 100 != 0 {
            result.push_str("and ");
            result.push_str(&number_to_words(n % 100));
        }
    } else if n >= 20 {
        result.push_str(&tens[(n / 10) as usize]);
        if n % 10 != 0 {
            result.push_str("-");
            result.push_str(&below_20[(n % 10) as usize]);
        }
    } else {
        result.push_str(&below_20[n as usize]);
    }

    result.trim().replace("  ", " ").replace(" -", "-").to_string()
}