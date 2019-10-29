// 1209. Remove All Adjacent Duplicates in String II
// Medium
// Given a string s, a k duplicate removal consists of choosing k adjacent and
// equal letters from s and removing them causing the left and the right side of
// the deleted substring to concatenate together. We repeatedly make k duplicate
// removals on s until we no longer can. Return the final string after all such
// duplicate removals have been made. It is guaranteed that the answer is
// unique. Example 1:
// Input: s = "abcd", k = 2
// Output: "abcd"
// Explanation: There's nothing to delete.
// Example 2:
// Input: s = "deeedbbcccbdaa", k = 3
// Output: "aa"
// Explanation:
// First delete "eee" and "ccc", get "ddbbbdaa"
// Then delete "bbb", get "dddaa"
// Finally delete "ddd", get "aa"
// Example 3:
// Input: s = "pbbcggttciiippooaais", k = 2
// Output: "ps"

use std::iter;
pub fn remove_duplicates(mut s: String, k: i32) -> String {
    let k = k as usize;
    loop {
        let mut new_s = "".to_string();
        for group in CharGroups::new(&s) {
            let add = &iter::repeat(group.chars().next().unwrap())
                .take(group.len() % k)
                .collect::<String>();
            new_s.push_str(add);
        }
        if new_s == s {
            return new_s;
        }
        s = new_s;
    }
}

pub struct CharGroups<'a> {
    input: &'a str,
}

impl<'a> CharGroups<'a> {
    pub fn new(input: &'a str) -> CharGroups<'a> {
        CharGroups { input }
    }
}

impl<'a> Iterator for CharGroups<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        self.input.chars().next().map(|c| {
            let end = self
                .input
                .find(|x| x != c)
                .unwrap_or_else(|| self.input.len());
            let (s, e) = self.input.split_at(end);
            self.input = e;
            s
        })
    }
}
