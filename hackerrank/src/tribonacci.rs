// 1137. N-th Tribonacci Number
// Easy

// The Tribonacci sequence Tn is defined as follows:

// T0 = 0, T1 = 1, T2 = 1, and Tn+3 = Tn + Tn+1 + Tn+2 for n >= 0.

// Given n, return the value of Tn.

// Example 1:

// Input: n = 4
// Output: 4
// Explanation:
// T_3 = 0 + 1 + 1 = 2
// T_4 = 1 + 1 + 2 = 4

// Example 2:

// Input: n = 25
// Output: 1389537

pub fn tribonacci(n: i32) -> i32 {
    let mut t = vec![0, 1, 1];
    let n = n as usize;
    for i in 3..=n {
        t.push(t[i - 1] + t[i - 2] + t[i - 3]);
    }
    t[n]
}
