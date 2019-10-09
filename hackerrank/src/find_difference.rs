// https://leetcode.com/problems/find-the-difference/
pub fn find_the_difference(s: String, t: String) -> char {
    use std::collections::HashMap;
    let mut map = s.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });
    let mut ret = None;
    for c in t.chars() {
        match map.get_mut(&c) {
            Some(e) => {
                if *e > 0 {
                    *e -= 1;
                } else {
                    ret = Some(c);
                    break;
                }
            }
            None => {
                ret = Some(c);
                break;
            }
        }
    }
    ret.unwrap()
}
