pub fn is_operator(token: char) -> bool {
    vec!['+', '-', '*', '/'].contains(&token)
}

pub fn is_digit(token: char) -> bool {
    token.is_digit(10)
}

pub fn is_brackets(token: char) -> bool {
    ['(', ')'].contains(&token)
}

pub fn is_open_bracket(token: char) -> bool {
    token == '('
}

pub fn is_close_bracket(token: char) -> bool {
    token == ')'
}

pub fn compare_operators(token1: char, token2: char) -> i8 {
    let w1 = get_weight_of_operator(token1);
    let w2 = get_weight_of_operator(token2);

    return if w2 > w1 {
        -1
    } else if w2 < w1 {
        1
    } else {
        0
    };
}

fn get_weight_of_operator(token: char) -> u8 {
    if ['*', '/'].contains(&token) {
        return 2;
    }

    if ['+', '-'].contains(&token) {
        return 1;
    }

    return 0;
}
