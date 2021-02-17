use super::lexer;
use super::translator;

pub fn parse_rpn_tokens(s: String) -> Vec<char> {
    let infix_tokens = lexer::parse_tokens(input);
    let rpn_tokens = translator::translator::translate_infix_to_rpn(infix_tokens);

    rpn_tokens
}
