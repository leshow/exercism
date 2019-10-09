// Given two strings s and t , write a function to determine if t is an anagram
// of s.
//
// Example 1:
//
// Input: s = "anagram", t = "nagaram"
// Output: true
//
// Example 2:
//
// Input: s = "rat", t = "car"
// Output: false
//
// Note:
// You may assume the string contains only lowercase alphabets.
use std::collections::HashMap;

fn is_anagram(s: String, t: String) -> bool {
    let s_map = s.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });
    let t_map = t.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });
    s_map == t_map
}

fn is_anagram_2(s: String, t: String) -> bool {
    let mut sv = s.chars().collect::<Vec<char>>();
    sv.sort();
    let mut st = t.chars().collect::<Vec<char>>();
    st.sort();
    sv == st
}
