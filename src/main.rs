mod cli;
mod calculator;
mod translator;

use cli::{input, output};
use translator::{lexer};

fn main() {
    let input = input::welcome("Введите выражение:");

    let infix_tokens = lexer::parse_tokens(input);
    let rpn_tokens = translator::translator::translate_infix_to_rpn(infix_tokens);

    let calculated = calculator::calculator::calculate(rpn_tokens);

    output::output_double(calculated);
}
