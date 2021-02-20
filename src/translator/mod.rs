mod lexer;
mod operator;
mod processor;
mod token;

pub fn translate_infix_to_rpn(s: String) -> Vec<String> {
    let infix_tokens = lexer::parse_tokens(s);

    processor::process_translate(infix_tokens)
}

pub fn normalize_tokens(s: Vec<String>) -> Vec<String> {
    s.iter()
        .map(|t| {
            if *t == token::get_unary_minus() {
                token::get_minus()
            } else {
                String::from(t)
            }
        })
        .collect::<Vec<String>>()
}
