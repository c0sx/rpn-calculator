pub fn parse_tokens(s: String) -> Vec<String> {
    let valid_digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];
    let valid_tokens = ['+', '-', '*', '/', '(', ')'];

    let mut tokens: Vec<String> = Vec::new();
    let mut token = String::new();

    for c in s.chars() {
        if valid_digits.contains(&c) {
            token.push_str(&c.to_string());
            continue;
        }

        if valid_tokens.contains(&c) {
            if token.len() > 0 {
                tokens.push(token.to_owned());
            }

            token = String::new();
            tokens.push(c.to_string());
        }
    }

    match token.parse::<f64>() {
        Ok(t) => tokens.push(t.to_string()),
        Err(_) => panic!("Ошибка в выражении")
    }

    tokens
}
