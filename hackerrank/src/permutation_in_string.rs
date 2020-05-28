/// 567. Permutation in String
/// Medium
/// Given two strings s1 and s2, write a function to return true if s2 contains
/// the permutation of s1. In other words, one of the first string's
/// permutations is the substring of the second string.

pub fn check_inclusion(s1: String, s2: String) -> bool {
    use std::collections::HashMap;
    if s1.len() > s2.len() {
        return false;
    }
    let char_map = s1.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });
    let len = s1.len();
    for i in 0..=(s2.len() - len) {
        let found_map = s2[i..(i + len)].chars().fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });
        if char_map == found_map {
            return true;
        }
    }
    false
}

#[test]
fn check_inclusion_test() {
    assert!(check_inclusion("ab".into(), "eidbaooo".into()));
    assert!(!check_inclusion("ab".into(), "eidboaoo".into()));
}
