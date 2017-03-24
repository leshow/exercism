pub fn is_valid<S: AsRef<str>>(s: S) -> bool {
    if s.as_ref().chars().any(|c| !c.is_digit(10) && c != ' ') {
        return false;
    }
    s.as_ref()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .rev()
        .enumerate()
        .map(|(ind, digit)| {
            let mut new_digit = digit;
            if ind % 2 != 0 {
                new_digit = digit * 2
            }
            if digit > 9 {
                new_digit -= 9;
            }
            new_digit
        })
        .sum::<u32>() % 10 == 0
}
