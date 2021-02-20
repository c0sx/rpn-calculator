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
    match operation {
        "+" => operation::add(stack),
        "*" => operation::multiply(stack),
        "/" => operation::divide(stack),
        "-" => operation::subtract(stack),
        "~" => operation::reverse(stack),
        _ => panic!("Недопустимая операция"),
    }
}
