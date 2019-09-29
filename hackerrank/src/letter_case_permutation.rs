pub fn letter_case_permutation(s: String) -> Vec<String> {
    let mut ret = Vec::new();
    let s = s.chars().collect::<Vec<char>>();
    go(&s, 0, "".into(), &mut ret);
    ret
}

fn go(s: &[char], i: usize, r: String, v: &mut Vec<String>) {
    if i == s.len() {
        return;
    }
    if s[i].is_alphabetic() {
        go(s, i + 1, format!("{}{}", r, s[i].to_uppercase()), v);
        go(s, i + 1, format!("{}{}", r, s[i].to_lowercase()), v);
    } else {
        go(s, i + 1, format!("{}{}", s[i], r), v);
    }
}

// about the same speed/memory as recursive
fn letter_case(a: String) -> Vec<String> {
    let mut s: Vec<(String, usize)> = Vec::new();
    let a = a.chars().collect::<Vec<char>>();
    s.push(("".into(), 0));
    let mut ret = Vec::new();
    while !s.is_empty() {
        let (cur, i) = s.pop().unwrap();
        if i == a.len() {
            ret.push(cur);
        } else if a[i].is_alphabetic() {
            s.push((format!("{}{}", cur, &a[i].to_uppercase()), i + 1));
            s.push((format!("{}{}", cur, &a[i].to_lowercase()), i + 1));
        } else {
            s.push((format!("{}{}", cur, a[i]), i + 1));
        }
    }
    ret
}
