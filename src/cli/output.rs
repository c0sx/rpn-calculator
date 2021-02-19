pub fn output_string(s: &str) {
    println!("{}", s);
}

pub fn output_double(d: f64) {
    println!("{}", d);
}

pub fn output_results(tokens: Vec<char>, result: f64) {
    let notation = tokens
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", notation);
    println!("{}", result);
}
