// 136. Single Number
// Easy
//
// Given a non-empty array of integers, every element appears twice except for
// one. Find that single one.
//
// Note:
//
// Your algorithm should have a linear runtime complexity. Could you implement
// it without using extra memory?
//
// Example 1:
//
// Input: [2,2,1]
// Output: 1
//
// Example 2:
//
// Input: [4,1,2,1,2]
// Output: 4

pub fn single_number(nums: Vec<i32>) -> Option<i32> {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for num in &nums {
        *map.entry(num).or_insert(0) += 1;
    }
    for (&k, v) in map {
        if v == 1 {
            return Some(k);
        }
    }
    None
}
