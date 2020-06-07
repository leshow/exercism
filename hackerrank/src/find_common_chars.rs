// 1002. Find Common Characters
// Easy
//
// Given an array A of strings made only from lowercase letters, return a list
// of all characters that show up in all strings within the list (including
// duplicates).  For example, if a character occurs 3 times in all strings but
// not 4 times, you need to include that character three times in the final
// answer.
//
// You may return the answer in any order.
// Example 1:
//
// Input: ["bella","label","roller"]
// Output: ["e","l","l"]
//
// Example 2:
// Input: ["cool","lock","cook"]
// Output: ["c","o"]

fn common_chars(arr: Vec<String>) -> Vec<char> {
    use std::cmp;
    let mut ret = Vec::new();
    for s in &arr {
        let mut common = [0; 26];
        for c in s.chars() {
            let pos = (c as u8 - b'a') as usize;
            common[pos] += 1;
        }
        ret.push(common);
    }
    let mut r = vec![];
    for i in 0..26 {
        let mut min = usize::max_value();
        for alphas in &ret {
            min = cmp::min(min, alphas[i]);
        }
        (0..min).for_each(|_| r.push((i as u8 + b'a') as char));
    }
    r
}

#[test]
fn test_chars() {
    let out = common_chars(["bella".into(), "label".into(), "roller".into()].to_vec());
    let expected = ['l', 'l', 'e'].to_vec();
    assert_eq!(out.len(), expected.len());
    for c in out {
        assert!(expected.contains(&c));
    }
}

#[test]
fn test_common() {
    let out = common_chars(["cool".into(), "lock".into(), "cook".into()].to_vec());
    let expected = ['c', 'o'].to_vec();
    assert_eq!(out.len(), expected.len());
    for c in out {
        assert!(expected.contains(&c));
    }
}
