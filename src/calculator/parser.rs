pub fn to_f64(c: char) -> f64 {
    let digit = c.to_digit(10);

    match digit {
        Some(d) => f64::from(d),
        None => panic!("Параметры для операций должны быть заданы числами")
    }
}
