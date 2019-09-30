// 338. Counting Bits
// Medium
//
// Given a non negative integer number num. For every numbers i in the range 0 ≤
// i ≤ num calculate the number of 1's in their binary representation and return
// them as an array.
//
// Example 1:
//
// Input: 2
// Output: [0,1,1]
//
// Example 2:
//
// Input: 5
// Output: [0,1,1,2,1,2]

pub fn count_bits(num: i32) -> Vec<i32> {
    let mut ret = Vec::with_capacity(num as usize + 1);
    for i in 0..=num {
        let mut i = i;
        let mut count = 0;
        while i > 0 {
            i &= i - 1;
            count += 1;
        }
        ret.push(count);
    }
    ret
}

pub fn count_bits_dp(num: i32) -> Vec<i32> {
    if num == 0 {
        return vec![num];
    } else if num == 1 {
        return vec![0, 1];
    }

    let mut bits = vec![0; num as usize + 1];
    bits[1] = 1;
    let mut lastpow2 = 0;

    for i in 2..=(num as usize) {
        if i & (i - 1) == 0 {
            lastpow2 = i;
            bits[i] = 1;
        } else {
            bits[i] = bits[lastpow2] + bits[i - lastpow2];
        }
    }
    bits
}
