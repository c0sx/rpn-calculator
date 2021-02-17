pub fn parse_tokens(s: String) -> Vec<char> {
    let valid_chars = vec![
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '-', '*', '/', '(', ')',
    ];

    return s
        .trim()
        .chars()
        .into_iter()
        .filter(|c| valid_chars.contains(c))
        .collect::<Vec<char>>();
}
