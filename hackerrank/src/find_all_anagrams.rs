/// 438. Find All Anagrams in a String
/// Medium
/// Given a string s and a non-empty string p, find all the start indices of p's
/// anagrams in s. Strings consists of lowercase English letters only and the
/// length of both strings s and p will not be larger than 20,100. The order of
/// output does not matter.

pub fn find_anagram(s: String, p: String) -> Vec<i32> {
    use std::collections::HashMap;
    if s.len() < p.len() {
        return vec![];
    }
    let char_map = p.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });
    let mut ret = vec![];
    let len = p.len();
    for i in 0..=(s.len() - len) {
        let found_map = s[i..(i + len)].chars().fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });
        if char_map == found_map {
            ret.push(i as i32);
        }
    }
    ret
}

#[test]
fn check_inclusion_test() {
    assert_eq!(find_anagram("cbaebabacd".into(), "abc".into()), vec![0, 6]);
}

#[test]
fn check_inclusion_test_b() {
    assert_eq!(find_anagram("abab".into(), "ab".into()), vec![0, 1, 2]);
}
