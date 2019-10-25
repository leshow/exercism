// 1177. Can Make Palindrome from Substring
// Medium

// Given a string s, we make queries on substrings of s.

// For each query queries[i] = [left, right, k], we may rearrange the substring
// s[left], ..., s[right], and then choose up to k of them to replace with any
// lowercase English letter.

// If the substring is possible to be a palindrome string after the operations
// above, the result of the query is true. Otherwise, the result is false.

// Return an array answer[], where answer[i] is the result of the i-th query
// queries[i].

// Note that: Each letter is counted individually for replacement so if for
// example s[left..right] = "aaa", and k = 2, we can only replace two of the
// letters.  (Also, note that the initial string s is never modified by any
// query.)

// Example :

// Input: s = "abcda", queries = [[3,3,0],[1,2,0],[0,3,1],[0,3,2],[0,4,1]]
// Output: [true,false,false,true,true]
// Explanation:
// queries[0] : substring = "d", is palidrome.
// queries[1] : substring = "bc", is not palidrome.
// queries[2] : substring = "abcd", is not palidrome after replacing only 1
// character. queries[3] : substring = "abcd", could be changed to "abba" which
// is palidrome. Also this can be changed to "baab" first rearrange it "bacd"
// then replace "cd" with "ab". queries[4] : substring = "abcda", could be
// changed to "abcba" which is palidrome.

// works but TLE
pub fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool> {
    queries
        .into_iter()
        .map(|q| make_pali(&s[(q[0] as usize)..=(q[1] as usize)], q[2] as usize))
        .collect::<Vec<_>>()
}

// the insight here is that any string that is a palindrome contains an even
// number of each character ex. "abba" , so if we have "abca" we can count all
// the characters, add up the % 2 for each to see how many we would need to add
// to make it a palindrome, then divide by 2 because we only need to change one
// of them

// this number will be the number of changes we need to make to turn the string
// to a palindrome
fn make_pali(s: &str, k: usize) -> bool {
    use std::collections::HashMap;
    let char_map: HashMap<char, usize> = s.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });
    (char_map.iter().fold(0usize, |acc, (_, v)| acc + (v % 2)) / 2) <= k
}
