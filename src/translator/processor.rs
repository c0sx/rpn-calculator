use crate::translator::{operator, token};

pub fn process_translate(tokens: Vec<String>) -> Vec<String> {
    let mut output_queue: Vec<String> = Vec::new();
    let mut stack: Vec<String> = Vec::new();

    process_tokens(tokens, &mut output_queue, &mut stack);
    process_remains_in_stack(&mut output_queue, &mut stack);

    output_queue
}

fn process_tokens(tokens: Vec<String>, output_queue: &mut Vec<String>, stack: &mut Vec<String>) {
    for token in tokens {
        if token::is_numeric(&token) {
            move_when_numeric(output_queue, token);
        } else if token::is_operator(&token) {
            move_when_operator(token, output_queue, stack);
        } else if token::is_open_bracket(&token) {
            move_when_open_bracket(stack, token);
        } else if token::is_close_bracket(&token) {
            move_when_close_bracket(output_queue, stack);
        }
    }
}

fn move_when_numeric(output_queue: &mut Vec<String>, token: String) {
    output_queue.push(token);
}

fn move_when_operator(token: String, output_queue: &mut Vec<String>, stack: &mut Vec<String>) {
    while let Some(last) = stack.last() {
        if token::is_operator(last) == false {
            break;
        }

        let comparison = operator::compare_operators(last, &token);
        if comparison == -1 {
            break;
        }

        output_queue.push(last.clone());
        stack.pop();
    }

    stack.push(token);
}

fn move_when_open_bracket(stack: &mut Vec<String>, token: String) {
    stack.push(token)
}

fn move_when_close_bracket(output_queue: &mut Vec<String>, stack: &mut Vec<String>) {
    let mut open_brackets_counter = 0;

    while let Some(token) = stack.pop() {
        if token::is_open_bracket(&token) == false {
            output_queue.push(token);
        } else {
            open_brackets_counter += 1;
            break;
        }
    }

    if open_brackets_counter == 0 {
        panic!("Пропущена открывающая скобка")
    }
}

fn process_remains_in_stack(output_queue: &mut Vec<String>, stack: &mut Vec<String>) {
    while let Some(token) = stack.pop() {
        if token::is_brackets(&token) {
            panic!("Присутствует незакрытая скобка")
        }

        output_queue.push(token)
    }
}
