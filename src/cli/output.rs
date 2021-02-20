pub fn output_string(s: &str) {
    println!("{}", s);
}

pub fn output_results(tokens: Vec<String>, result: f64) {
    println!("{}", tokens.join(" "));
    println!("{}", result);
}
