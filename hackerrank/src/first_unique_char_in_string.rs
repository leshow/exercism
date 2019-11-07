// 387. First Unique Character in a String
// Easy

// Given a string, find the first non-repeating character in it and return it's
// index. If it doesn't exist, return -1.

// Examples:

// s = "leetcode"
// return 0.

// s = "loveleetcode",
// return 2.

pub fn first_uniq_char(s: String) -> i32 {
    use std::collections::HashMap;
    let s = s.as_bytes();
    let map = s.iter().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });

    for i in s.iter() {
        if let Some(&count) = map.get(i) {
            if count == 1 {
                return *i as i32;
            }
        }
    }
    -1
}
