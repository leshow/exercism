pub fn number<S: AsRef<str>>(user_number: S) -> Option<String> {
    let clean = user_number
        .as_ref()
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<Vec<char>>();

    match clean.len() {
        11 if clean[0] == '1' && clean[4] == '0'
            || clean[4] == '1'
            || clean[1] == '0'
            || clean[1] == '1' =>
        {
            None
        }
        11 if clean[0] == '1' => Some(clean[1..].iter().collect::<String>()),
        10 if clean[0] == '0' || clean[0] == '1' || clean[3] == '1' || clean[3] == '0' => None,
        10 => Some(clean.into_iter().collect::<String>()),
        _ => None,
    }
}
