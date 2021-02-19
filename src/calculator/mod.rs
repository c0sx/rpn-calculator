mod operation;
mod parser;

pub fn calculate(queue: &mut Vec<char>) -> f64 {
    let mut stack: Vec<f64> = Vec::new();

    for token in queue {
        let token = *token;

        if operation::is_operation(token) == false {
            stack.push(parser::to_f64(token));
            continue;
        }

        let (a, b) = get_arguments(&mut stack);
        let result = match token {
            '+' => operation::add(b, a),
            '-' => operation::subtract(b,a),
            '*' => operation::multiply(b, a),
            '/' => operation::divide(b, a),
            _ => panic!("Недопустимая операция")
        };

        stack.push(result);
    }

    let result = stack.pop();
    match result {
        Some(result) => result,
        None => panic!("Возникла ошибка при вычислении")
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
        None => 0.0
    }
}
