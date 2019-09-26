// leetcode
// Implement atoi which converts a string to an integer.
//
// The function first discards as many whitespace characters as necessary until
// the first non-whitespace character is found. Then, starting from this
// character, takes an optional initial plus or minus sign followed by as many
// numerical digits as possible, and interprets them as a numerical value.
//
// The string can contain additional characters after those that form the
// integral number, which are ignored and have no effect on the behavior of
// this function.
//
// If the first sequence of non-whitespace characters in str is not a valid
// integral number, or if no such sequence exists because either str is empty
// or it contains only whitespace characters, no conversion is performed.
//
// If no valid conversion could be performed, a zero value is returned.
//
// Note:
//
//     Only the space character ' ' is considered as whitespace character.
//     Assume we are dealing with an environment which could only store
// integers within the 32-bit signed integer range: [−231,  231 − 1]. If the
// numerical value is out of the range of representable values, INT_MAX (231 −
// 1) or INT_MIN (−231) is returned.
pub fn my_atoi(str: String) -> i32 {
    let mut num: Option<i32> = Some(0);
    let mut neg = false;
    for (i, c) in str.trim().chars().enumerate() {
        if i == 0 && (c == '-' || c == '+') {
            neg = c == '-';
            continue;
        } else if i == 0 && c.is_alphabetic() {
            break;
        }
        match c {
            '0'..='9' => {
                let cur = (c as u32 - '0' as u32) as i32;
                num = num.and_then(|m| m.checked_mul(10).and_then(|n| n.checked_add(cur)));
            }
            _ => break,
        }
    }
    convert(num, neg)
}

fn convert(v: Option<i32>, do_neg: bool) -> i32 {
    match v {
        None if do_neg => std::i32::MIN,
        None => std::i32::MAX,
        Some(v) if do_neg => {
            let (neg, overflow) = (v as i32).overflowing_neg();
            if overflow {
                std::i32::MIN
            } else {
                neg
            }
        }
        Some(v) => v,
    }
}
