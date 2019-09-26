// 1017. Convert to Base -2
// Medium

// Given a number N, return a string consisting of "0"s and "1"s that represents
// its value in base -2 (negative two).

// The returned string must have no leading zeroes, unless the string is "0".

// Example 1:

// Input: 2
// Output: "110"
// Explantion: (-2) ^ 2 + (-2) ^ 1 = 2

// Example 2:

// Input: 3
// Output: "111"
// Explantion: (-2) ^ 2 + (-2) ^ 1 + (-2) ^ 0 = 3

// Example 3:

// Input: 4
// Output: "100"
// Explantion: (-2) ^ 2 = 4

pub fn base_neg2(n: i32) -> String {
    let mut s = "".into();
    rec(n, &mut s);
    s
}
fn rec(n: i32, s: &mut String) {
    if n == 0 {
        return s.push('0');
    }
    if n == 1 {
        return s.push('1');
    }
    if n & 1 == 1 {
        rec(-(n >> 1), s);
        s.push('1')
    } else {
        rec(-(n >> 1), s);
        s.push('0')
    }
}

#[test]
fn binary_string() {
    assert_eq!(base_neg2(4), "100");
}
