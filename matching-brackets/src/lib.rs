
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    let input = string.as_bytes();

    for &c in input {
        match c {
            b'(' | b'{' | b'[' => stack.push(c),
            b')' | b'}' | b']' => match stack.pop() {
                Some(s) if inverse(c) != s => return false,
                Some(_) => continue,
                _ => return false,
            },
            _ => continue,
        }
    }
    stack.is_empty()
}

pub fn inverse(b: u8) -> u8 {
    match b {
        b')' => b'(',
        b']' => b'[',
        b'}' => b'{',
        _ => panic!("cant match"),
    }
}