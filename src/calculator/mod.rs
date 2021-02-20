mod operation;
mod parser;

pub fn calculate(queue: &Vec<String>) -> f64 {
    let mut stack: Vec<f64> = Vec::new();

    for token in queue {
        if parser::is_argument(token) {
            let numeric = parser::to_f64(token.to_string());
            stack.push(numeric);
            continue;
        }

        let result = evaluate(token, &mut stack);
        stack.push(result);
    }

    let result = stack.pop();
    match result {
        Some(result) => result,
        None => panic!("Возникла ошибка при вычислении"),
    }
}

fn evaluate(operation: &str, stack: &mut Vec<f64>) -> f64 {
    let (a, b) = get_arguments(stack);

     match operation {
        "+" => operation::add(b, a),
        "-" => operation::subtract(b, a),
        "*" => operation::multiply(b, a),
        "/" => operation::divide(b, a),
        _ => panic!("Недопустимая операция"),
    }
}

fn get_arguments(stack: &mut Vec<f64>) -> (f64, f64) {
    let a = get_argument(stack.pop());
    let b = get_argument(stack.pop());

    (a, b)
}

fn get_argument(c: Option<f64>) -> f64 {
    match c {
        Some(c) => c,
        None => 0.0,
    }
}
