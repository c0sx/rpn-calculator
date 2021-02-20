use super::token;

pub fn parse_tokens(s: String) -> Vec<String> {
    let numeric = token::get_numeric_tokens();
    let separators = token::get_not_numeric_tokens();

    let s = remove_whitespaces(s);

    let mut cursor = 0;
    let mut tokens: Vec<String> = Vec::new();
    while let Some(token) = get_next_token(&s, cursor, &tokens, &numeric, &separators) {
        cursor += token.len();
        tokens.push(token);
    }

    tokens
}

fn remove_whitespaces(s: String) -> String {
    let without_whitespaces = s.split(" ");

    without_whitespaces.collect::<Vec<&str>>().join("")
}

fn get_next_token(
    s: &String,
    start: usize,
    tokens: &Vec<String>,
    numeric: &Vec<char>,
    separators: &Vec<char>,
) -> Option<String> {
    if start >= s.len() {
        return None;
    }

    let slice = &s[start..];
    let token = parse_token(slice, numeric, separators);

    if token.len() > 0 {
        return if is_unary(&token, tokens) {
            Some(token::get_unary_minus())
        } else {
            Some(token)
        };
    } else {
        None
    }
}

fn parse_token(slice: &str, numeric: &Vec<char>, separators: &Vec<char>) -> String {
    let mut token = String::new();

    for c in slice.chars().into_iter() {
        if numeric.contains(&c) {
            token.push(c);
            continue;
        }

        if separators.contains(&c) {
            if token.len() == 0 {
                token.push(c);
            }

            break;
        }
    }

    token
}

fn is_unary(token: &String, tokens: &Vec<String>) -> bool {
    if token::is_minus_operator(token) == false {
        return false;
    }

    let prev = tokens.last();
    let operators = token::get_operators()
        .iter()
        .map(|op| op.to_string())
        .collect::<Vec<String>>();

    match prev {
        Some(prev) => operators.contains(&prev),
        None => true,
    }
}
