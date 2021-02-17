use std::io;

use super::output;

pub fn welcome(message: &str) -> String {
    output::output_string(message);

    get_input()
}

fn get_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("При обработки ввода возникла ошибка");

    input
}
