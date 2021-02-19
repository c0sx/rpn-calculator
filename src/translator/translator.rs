pub fn translate_infix_to_rpn(tokens: Vec<char>) -> Vec<char> {
    let mut output_queue: Vec<char> = Vec::new();
    let mut stack: Vec<char> = Vec::new();

    process_tokens(tokens, &mut output_queue, &mut stack);
    process_remains_in_stack(&mut output_queue, &mut stack);

    output_queue
}

fn process_tokens(tokens: Vec<char>, output_queue: &mut Vec<char>, stack: &mut Vec<char>) {
    for c in tokens {
        if c.is_digit(10) {
            output_queue.push(c);
        } else if is_operator(c) {
            move_when_operator(c, output_queue, stack);
        } else if is_open_bracket(c) {
            stack.push(c);
        } else if is_close_bracket(c) {
            move_when_close_bracket(output_queue, stack);
        }
    }
}

fn is_operator(c: char) -> bool {
    vec!['+', '-', '*', '/'].contains(&c)
}

fn move_when_operator(c: char, output_queue: &mut Vec<char>, stack: &mut Vec<char>) {
    loop {
        let token = stack.last();
        let token = match token {
            Some(c) => c,
            None => break,
        };

        if is_operator(*token) == false {
            break;
        }

        let comparison = compare_operators(*token, c);
        if comparison >= 0 {
            output_queue.push(*token);
            stack.pop();
        } else {
            break;
        }
    }

    stack.push(c);
}

fn compare_operators(c1: char, c2: char) -> i8 {
    let w1 = get_weight_of_token(c1);
    let w2 = get_weight_of_token(c2);

    return if w2 > w1 {
        -1
    } else if w2 < w1 {
        1
    } else {
        0
    };
}

fn get_weight_of_token(c: char) -> u8 {
    if ['*', '/'].contains(&c) {
        return 2;
    }

    if ['+', '-'].contains(&c) {
        return 1;
    }

    return 0;
}

fn is_open_bracket(c: char) -> bool {
    c == '('
}

fn is_close_bracket(c: char) -> bool {
    c == ')'
}

fn move_when_close_bracket(output_queue: &mut Vec<char>, stack: &mut Vec<char>) {
    let mut open_brackets_counter = 0;

    while let Some(token) = stack.pop() {
        if is_open_bracket(token) == false {
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
    loop {
        let token = stack.pop();
        let token = match token {
            Some(t) => t,
            None => return,
        };

        if is_open_bracket(token) || is_close_bracket(token) {
            panic!("Присутствует незакрытая скобка")
        }

        output_queue.push(token)
    }
}
