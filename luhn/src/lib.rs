pub fn is_valid(s: &str) -> bool {
    if s.chars().filter(|c| c.is_digit(10)).count() <= 1 {
        return false;
    }
    if s.chars().any(|c| !c.is_digit(10) && c != ' ') {
        return false;
    }

    let sum = s.chars()
        .filter_map(|c| c.to_digit(10))
        .rev()
        .enumerate()
        .map(|(index, mut digit)| {
            if index % 2 != 0 {
                digit = digit * 2
            }
            if digit > 9 {
                digit = digit - 9
            }
            digit
        })
        .sum::<u32>();

    return sum % 10 == 0;
}
