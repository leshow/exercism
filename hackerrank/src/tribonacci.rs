pub fn tribonacci(n: i32) -> i32 {
    let mut t = vec![0, 1, 1];
    let n = n as usize;
    for i in 3..=n {
        t.push(t[i - 1] + t[i - 2] + t[i - 3]);
    }
    t[n]
}
