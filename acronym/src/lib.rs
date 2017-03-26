pub fn abbreviate(input: &str) -> String {
    input.split(|c: char| c.is_whitespace() || !c.is_alphanumeric())
        .filter_map(|word| word.to_uppercase().chars().nth(0))
        .collect::<String>()
}
