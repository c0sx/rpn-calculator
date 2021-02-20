mod lexer;
mod operator;
mod processor;
mod token;

pub fn translate_infix_to_rpn(s: String) -> Vec<String> {
    let infix_tokens = lexer::parse_tokens(s);

    processor::process_translate(infix_tokens)
}
