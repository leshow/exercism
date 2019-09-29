pub fn min_ascii_delete_sum(s1: String, s2: String) -> i32 {
    use std::cmp;
    let s1 = s1.chars().map(|c| c as u8 as i32).collect::<Vec<_>>();
    let s2 = s2.chars().map(|c| c as u8 as i32).collect::<Vec<_>>();
    let n = s1.len();
    let m = s2.len();
    let mut t = vec![vec![0; m + 1]; n + 1];
    for i in 1..=n {
        t[i][0] = t[i - 1][0] + s1[i - 1];
    }
    for i in 1..=m {
        t[0][i] = t[0][i - 1] + s2[i - 1];
    }
    for i in 1..=n {
        for j in 1..=m {
            if s1[i - 1] == s2[j - 1] {
                t[i][j] = t[i - 1][j - 1];
            } else {
                t[i][j] = cmp::min(t[i - 1][j] + s1[i - 1], t[i][j - 1] + s2[j - 1])
            }
        }
    }
    t[n][m]
}

// or done by getting the longest common subsequence, then subtracting from each
// of the string sums
pub fn min_ascii_delete_sum_lcs(s1: String, s2: String) -> i32 {
    use std::cmp;
    let s1 = s1.chars().map(|c| c as u8 as i32).collect::<Vec<_>>();
    let s2 = s2.chars().map(|c| c as u8 as i32).collect::<Vec<_>>();
    let n = s1.len();
    let m = s2.len();
    let mut t = vec![vec![0; m + 1]; n + 1];
    for i in 0..=n {
        for j in 0..=m {
            if i == 0 || j == 0 {
                t[i][j] = 0;
            } else if s1[i - 1] == s2[j - 1] {
                t[i][j] = s1[i - 1] + t[i - 1][j - 1];
            } else {
                t[i][j] = cmp::max(t[i - 1][j], t[i][j - 1])
            }
        }
    }
    s1.iter().sum::<i32>() + s2.iter().sum::<i32>() - 2 * t[n][m]
}
