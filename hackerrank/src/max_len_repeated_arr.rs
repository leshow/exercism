// 718. Maximum Length of Repeated Subarray
// Medium

// Given two integer arrays A and B, return the maximum length of an subarray
// that appears in both arrays.

// Example 1:

// Input:
// A: [1,2,3,2,1]
// B: [3,2,1,4,7]
// Output: 3
// Explanation:
// The repeated subarray with maximum length is [3, 2, 1].

// Brute force: time limit expire
pub fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
    use std::cmp;
    let mut max_len = 0;
    for i in 0..a.len() {
        for j in 1..=(a.len() - i) {
            let slice = &a[i..(i + j)];
            for n in 0..=(b.len() - j) {
                let bslice = &b[n..(n + j)];
                if slice == bslice {
                    max_len = cmp::max(j, max_len);
                }
            }
        }
    }
    max_len as i32
}

// DP
pub fn find_length_dp(a: Vec<i32>, b: Vec<i32>) -> i32 {
    use std::cmp;
    let mut t = vec![vec![0; b.len() + 1]; a.len() + 1];
    let mut max_len = 0;
    for i in 0..=a.len() {
        for j in 0..=b.len() {
            if i == 0 || j == 0 {
                t[i][j] = 0;
            } else if a[i - 1] == b[j - 1] {
                t[i][j] = t[i - 1][j - 1] + 1;
                max_len = cmp::max(max_len, t[i][j]);
            }
        }
    }
    max_len
}
