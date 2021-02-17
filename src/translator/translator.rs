pub fn translate_infix_to_rpn(tokens: Vec<char>) -> Vec<char> {
    let mut output_queue: Vec<char> = Vec::new();
    let mut stack: Vec<char> = Vec::new();

    process_tokens(tokens, &mut output_queue, &mut stack);
    process_remains_in_stack(&mut stack);

    Vec::new()
}

fn process_tokens(tokens: Vec<char>, output_queue: &mut Vec<char>, stack: &mut Vec<char>) {
    for c in tokens {
        if c.is_digit(10) {
            output_queue.push(c);
            continue;
        }

        if is_operator(c) {
            // move_when_operator()
            continue;
        }

        if is_open_bracket(c) {
            stack.push(c);
            continue;
        }

        if is_close_bracket(c) {
            // move_when_close_bracket()
            continue;
        }
    }
}

fn is_operator(c: char) -> bool {
    vec!['+', '-', '*', '/'].contains(&c)
}

fn is_low_priority_operator(c: char) -> bool {
    vec!['+', '-'].contains(&c)
}

fn is_high_priority_operator(c: char) -> bool {
    vec!['*', '/'].contains(&c)
}

fn get_operator_priority(c: char) -> u8 {
    if is_low_priority_operator(c) {
        return 0;
    }

    return 1;
}

fn is_open_bracket(c: char) -> bool {
    c == '('
}

fn is_close_bracket(c: char) -> bool {
    c == ')'
}

fn process_remains_in_stack(stack: &mut Vec<char>) {

}
