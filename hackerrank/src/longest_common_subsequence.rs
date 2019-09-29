pub fn longest_common_subsequence(t1: String, t2: String) -> i32 {
    use std::cmp;
    let t1 = t1.as_bytes();
    let t2 = t2.as_bytes();
    println!("{:#?} {:#?}", t1, t2);
    let mut t = vec![vec![0; t2.len() + 1]; t1.len() + 1];
    println!("{:?}", t);
    for i in 0..=t1.len() {
        for j in 0..=t2.len() {
            if i == 0 || j == 0 {
                t[i][j] = 0;
            } else if t1[i - 1] == t2[j - 1] {
                t[i][j] = 1 + t[i - 1][j - 1];
            } else {
                t[i][j] = cmp::max(t[i - 1][j], t[i][j - 1]);
            }
        }
    }
    t[t1.len()][t2.len()]
}

#[test]
fn test_lcs() {
    assert_eq!(longest_common_subsequence("abcde".into(), "ace".into()), 3);
}
