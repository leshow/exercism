// 76. Minimum Window Substring
// Hard
// Given a string S and a string T, find the minimum window in S which will
// contain all the characters in T in complexity O(n). Example:
// Input: S = "ADOBECODEBANC", T = "ABC"
// Output: "BANC"
// Note:
//     If there is no such window in S that covers all characters in T, return
// the empty string "".     If there is such window, you are guaranteed that
// there will always be only one unique minimum window in S.

// naive:
pub fn min_window_naive(s: String, t: String) -> String {
    use std::collections::HashMap;
    let mut min_len = usize::max_value();
    let mut min = "".into();
    let k = t.len();
    let tmap = t.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });
    for i in 0..s.len() {
        for j in k..=(s.len() - i) {
            let map = s[i..(i + j)].chars().fold(HashMap::new(), |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            });
            let mut found = true;
            for (k, v) in tmap.iter() {
                match map.get(k) {
                    None => {
                        found = false;
                        break;
                    }
                    Some(w) if w < v => {
                        found = false;
                        break;
                    }
                    _ => continue,
                }
            }
            if found && min_len > j {
                min_len = j;
                min = s[i..(i + j)].into();
            }
        }
    }
    min
}

// https://leetcode.com/problems/minimum-window-substring
//
// We can use a simple sliding window approach to solve this problem.

// In any sliding window based problem we have two pointers. One right
// pointer whose job is to expand the current window and then we have the
// left pointer whose job is to contract a given window. At any point in
// time only one of these pointers move and the other one remains fixed.

// The solution is pretty intuitive. We keep expanding the window by moving the
// right pointer. When the window has all the desired characters, we contract
// (if possible) and save the smallest window till now.

// The answer is the smallest desirable window.
// Algorithm

//     1. We start with two pointers, left and right initially
// pointing to the first element of the string SSS.

//     2. We use the right pointer to expand the window until we get a
// desirable window i.e. a window that contains all of the characters of TTT.

//     3. Once we have a window with all the characters, we can move the left
// pointer ahead one by one. If the window is still a desirable one we keep on
// updating the minimum window size.

//     4. If the window is not desirable any more, we repeat step2
// onwards.

pub fn min_window(s: String, t: String) -> String {
    use std::collections::HashMap;
    if s.is_empty() || t.is_empty() {
        return "".into();
    }
    let tmap = t.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });
    let mut formed = 0;
    let mut min_len = usize::max_value();
    let mut l = 0;
    let mut r = 0;
    let mut map = HashMap::new();
    let s = s.chars().collect::<Vec<_>>();
    let mut min = "".into();
    while r < s.len() {
        let ch = s[r];
        *map.entry(ch).or_insert(0) += 1;

        if tmap.contains_key(&ch) && map[&ch] == tmap[&ch] {
            formed += 1;
        }
        while l <= r && formed == tmap.len() {
            let remove = s[l];
            let slice_len = r - l + 1;
            if slice_len < min_len {
                min_len = slice_len;
                min = s[l..=r].iter().collect();
            }
            map.entry(remove).and_modify(|v| *v -= 1);
            if tmap.contains_key(&remove) && map[&remove] < tmap[&remove] {
                formed -= 1;
            }
            l += 1;
        }
        r += 1;
    }
    min
}

#[test]
fn test_min_win_sub() {
    assert_eq!(
        min_window("CODEBANC".into(), "ABC".into()),
        "BANC".to_string()
    );
}
