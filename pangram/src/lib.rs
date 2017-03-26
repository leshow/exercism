use std::collections::BTreeSet;

pub fn is_pangram(sentence: &str) -> bool {
    sentence.to_lowercase()
        .chars()
        .filter(|c| is_valid(*c))
        .collect::<BTreeSet<char>>()
        .len() == 26
}
fn is_valid(c: char) -> bool {
    let c = c as u8;
    b'a' <= c && c <= b'z'
}
