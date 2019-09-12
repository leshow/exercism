// https://leetcode.com/problems/is-subsequence/
pub fn is_subsequence(s: String, t: String) -> bool {
    let t = t.as_bytes();
    let s = s.as_bytes();
    let mut pos = 0;
    for &c in s {
        let mut found = false;
        for i in pos..t.len() {
            if t[i] == c {
                found = true;
                pos = i + 1;
                break;
            }
        }
        if !found {
            return false;
        }
    }
    true
}
