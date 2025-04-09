pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    let mut pyramid = Vec::new();

    for i in 1..=i {
        let sp = " ".repeat(i as usize);
        let c = v.repeat(i as usize);
        let n = format!("{}{}", sp, c);
        pyramid.push(n);
    }

    for i in (1..i).rev() {
        let sp = " ".repeat(i as usize);
        let c = v.repeat(i as usize);
        let n = format!("{}{}", sp, c);
        pyramid.push(n);
    }
    pyramid
}