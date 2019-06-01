
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    let input = string.as_bytes();

    for &c in input {
        match c {
            b'(' | b'{' | b'[' => stack.push(c),
            b')' | b'}' | b']' => {
                if let Some(s) = stack.pop() {
                    if inverse(c) != s {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => {
                continue;
            }
        }
    }
    if !stack.is_empty() {
        return false;
    }
    true
}

pub fn inverse(b: u8) -> u8 {
    match b {
        b')' => b'(',
        b']' => b'[',
        b'}' => b'{',
        _ => panic!("cant match"),
    }
}