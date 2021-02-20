pub fn get_numeric_tokens() -> Vec<char> {
    vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.']
}

pub fn get_not_numeric_tokens() -> Vec<char> {
    vec!['+', '-', '*', '/', '(', ')']
}

pub fn get_unary_minus() -> String {
    String::from("~")
}

pub fn get_operators() -> Vec<char> {
    vec!['+', '-', '*', '/', '(', ')', '~']
}

pub fn is_low_priority_operator_token(token: &String) -> bool {
    ['+', '-', '~']
        .iter()
        .any(|op| token.contains(&op.to_string()))
}

pub fn is_high_priority_operator_token(token: &String) -> bool {
    ['*', '/'].iter().any(|op| token.contains(&op.to_string()))
}

pub fn is_minus_operator(token: &String) -> bool {
    token == "-"
}

pub fn is_numeric(token: &String) -> bool {
    let test = token.parse::<f64>();
    match test {
        Ok(_) => true,
        Err(_) => false,
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
    ['+', '-', '*', '/', '~']
        .iter()
        .any(|op| token.contains(&op.to_string()))
}
