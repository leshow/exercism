pub fn rotate(input: &str, shift: u8) -> String {
    input.chars()
        .map(|c| {
            let case = if c.is_uppercase() { b'A' } else { b'a' };
            if c.is_alphabetic() {
                ((((c as u8) - case + shift) % 26) + case) as char
            } else {
                c
            }
        })
        .collect::<String>()
}
