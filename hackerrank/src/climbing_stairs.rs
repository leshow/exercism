/// https://leetcode.com/problems/climbing-stairs/
pub fn climb_stairs_dp(n: i32) -> i32 {
    let mut t = vec![0, 1, 2];
    let n = n as usize;
    if n <= 2 {
        return t[n];
    }
    t.reserve(n);
    for i in 3..=n {
        t.push(t[i - 1] + t[i - 2]);
    }
    t[n]
}
// recursive
pub fn climb_stairs(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else if n == 2 {
        1 + climb_stairs(n - 1)
    } else {
        climb_stairs(n - 1) + climb_stairs(n - 2)
    }
}
