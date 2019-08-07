/// leetcode
/// 7. Reverse Integer
/// Easy
///
/// Given a 32-bit signed integer, reverse digits of an integer.
///
/// Example 1:
///
/// Input: 123
/// Output: 321
///
/// Example 2:
///
/// Input: -123
/// Output: -321
///
/// Example 3:
///
/// Input: 120
/// Output: 21
///
/// Note:
/// Assume we are dealing with an environment which could only store integers
/// within the 32-bit signed integer range: [−231,  231 − 1]. For the purpose of
/// this problem, assume that your function returns 0 when the reversed integer
/// overflows.
fn reverse(num: i32) -> i32 {
    let mut res: Option<i32> = Some(0);
    let mut neg = false;
    let mut num = if num < 0 {
        neg = true;
        -num
    } else {
        num
    };
    while num != 0 {
        let digits = (num as f32).log10() as u32;
        let modulus = num % 10;
        num /= 10;
        let tens = 10_i32.pow(1 + digits);
        let modulus = modulus.checked_mul(tens);
        res = res.and_then(|r| modulus.and_then(|mods| r.checked_add(mods)));
    }
    match res {
        Some(v) if neg => -(v / 10),
        Some(v) => v / 10,
        None => 0,
    }
}
