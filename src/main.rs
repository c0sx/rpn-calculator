mod calculator;
mod cli;
mod translator;

use cli::{input, output};
use translator::processor;

fn main() {
    let input = input::welcome("Введите выражение:");

    let rpn_tokens = processor::parse_rpn_tokens(input);
    let calculated = calculator::calculator::calculate(rpn_tokens);

    output::output_double(calculated);
}
