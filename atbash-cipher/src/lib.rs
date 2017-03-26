use std::ascii::AsciiExt;

pub fn encode(input: &str) -> String {
    let encoded = input.to_lowercase()
        .chars()
        .filter(|c| c.is_ascii() && c.is_alphanumeric())
        .map(shift)
        .collect::<Vec<char>>();
    // need to convert each chunk (&[char]) into an iterator
    // then convert iterated type from &char to char with cloned()
    encoded.chunks(5)
        .map(|s| s.iter().cloned().collect())
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn decode(input: &str) -> String {
    input.chars()
        .filter(|c| *c != ' ')
        .map(shift)
        .collect::<String>()
}

fn shift(c: char) -> char {
    if c.is_digit(10) {
        c
    } else {
        (b'z' - (c as u8) + b'a') as char
    }
}
