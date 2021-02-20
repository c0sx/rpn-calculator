pub fn to_f64(s: String) -> f64 {
    let result = s.parse::<f64>();

    match result {
        Ok(ok) => ok,
        Err(_) => panic!("Параметры для операций должны быть заданы числами"),
    }
}

pub fn is_argument(token: &String) -> bool {
    is_operation(token) == false
}

fn is_operation(token: &String) -> bool {
    ['+', '-', '*', '/'].iter().any(|op| token.contains(&op.to_string()))
}
