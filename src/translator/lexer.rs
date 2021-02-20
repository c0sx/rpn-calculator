pub fn parse_tokens(s: String) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();

    let s = remove_whitespaces(s);

    let mut cursor = 0;
    while let Some(token) = get_next_token(&s, cursor) {
        cursor += token.len();

        if token == "-" {
            if let Some(prev) = tokens.last() {
                if ["+", "-", "*", "/", "(", ")"].contains(&prev.as_str()) {
                    tokens.push(String::from("~"));
                    continue;
                }
            } else {
                tokens.push(String::from("~"));
                continue;
            }
        }

        tokens.push(token);
    }

    tokens
}

fn remove_whitespaces(s: String) -> String {
    let without_whitespaces = s.split(" ");

    without_whitespaces.collect::<Vec<&str>>().join("")
}

fn get_next_token(s: &String, start: usize) -> Option<String> {
    let numeric = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];
    let separators = ['+', '-', '*', '/', '(', ')'];
    let mut token = String::new();

    if start >= s.len() {
        return None;
    }

    let slice = &s[start..];
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

    if token.len() > 0 {
        Some(token)
    } else {
        None
    }
}
