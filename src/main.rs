mod calculator;
mod cli;
mod translator;

use cli::{input, output};

fn main() {
    let input = input::welcome("Введите выражение:");

    let mut rpn_tokens = translator::translate_infix_to_rpn(input);
    let result = calculator::calculate(&mut rpn_tokens);

    output::output_results(rpn_tokens, result);
}
