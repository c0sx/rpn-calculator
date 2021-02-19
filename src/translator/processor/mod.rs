mod token;

pub fn process_translate(tokens: Vec<char>) -> Vec<char> {
    let mut output_queue: Vec<char> = Vec::new();
    let mut stack: Vec<char> = Vec::new();

    process_tokens(tokens, &mut output_queue, &mut stack);
    process_remains_in_stack(&mut output_queue, &mut stack);

    output_queue
}

fn process_tokens(tokens: Vec<char>, output_queue: &mut Vec<char>, stack: &mut Vec<char>) {
    for token in tokens {
        if token::is_digit(token) {
            output_queue.push(token);
        } else if token::is_operator(token) {
            move_when_operator(token, output_queue, stack);
        } else if token::is_open_bracket(token) {
            stack.push(token);
        } else if token::is_close_bracket(token) {
            move_when_close_bracket(output_queue, stack);
        }
    }
}

fn move_when_operator(token: char, output_queue: &mut Vec<char>, stack: &mut Vec<char>) {
    while let Some(last) = stack.last() {
        if token::is_operator(*last) == false {
            break;
        }

        let comparison = token::compare_operators(*last, token);
        if comparison == -1 {
            break;
        }

        output_queue.push(*last);
        stack.pop();
    }

    stack.push(token);
}

fn move_when_close_bracket(output_queue: &mut Vec<char>, stack: &mut Vec<char>) {
    let mut open_brackets_counter = 0;

    while let Some(token) = stack.pop() {
        if token::is_open_bracket(token) == false {
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

fn process_remains_in_stack(output_queue: &mut Vec<char>, stack: &mut Vec<char>) {
    while let Some(token) = stack.pop() {
        if token::is_brackets(token) {
            panic!("Присутствует незакрытая скобка")
        }

        output_queue.push(token)
    }
}
