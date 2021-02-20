pub fn is_numeric(token: &String) -> bool {
    let test = token.parse::<f64>();
    match test {
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn is_brackets(token: &String) -> bool {
    ["(", ")"].iter().any(|b| b == token)
}

pub fn is_open_bracket(token: &String) -> bool {
    token == "("
}

pub fn is_close_bracket(token: &String) -> bool {
    token == ")"
}

pub fn is_operator(token: &String) -> bool {
    ['+', '-', '*', '/'].iter().any(|op| token.contains(&op.to_string()))
}
