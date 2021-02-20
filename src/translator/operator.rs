use super::token;

pub fn compare_operators(token1: &String, token2: &String) -> i8 {
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

fn get_weight_of_operator(token: &String) -> u8 {
    if token::is_high_priority_operator_token(&token) {
        return 2;
    }

    if token::is_low_priority_operator_token(&token) {
        return 1;
    }

    return 0;
}
