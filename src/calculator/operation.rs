pub fn add(stack: &mut Vec<f64>) -> f64 {
    let a = get_argument(stack);
    let b = get_argument(stack);

    a + b
}

pub fn subtract(stack: &mut Vec<f64>) -> f64 {
    let a = get_argument(stack);
    let b = get_argument(stack);

    b - a
}

pub fn negative(stack: &mut Vec<f64>) -> f64 {
    let a = get_argument(stack);

    -a
}

pub fn multiply(stack: &mut Vec<f64>) -> f64 {
    let a = get_argument(stack);
    let b = get_argument(stack);

    a * b
}

pub fn divide(stack: &mut Vec<f64>) -> f64 {
    let a = get_argument(stack);
    let b = get_argument(stack);

    if a == 0.0 {
        panic!("Деление на ноль")
    }

    b / a
}

fn get_argument(stack: &mut Vec<f64>) -> f64 {
    let arg = stack.pop();

    match arg {
        Some(a) => a,
        None => panic!("Ошибка при выполнении"),
    }
}
