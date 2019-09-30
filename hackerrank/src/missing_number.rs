// 268. Missing Number
// Easy
//
// Given an array containing n distinct numbers taken from 0, 1, 2, ..., n, find
// the one that is missing from the array.
//
// Example 1:
//
// Input: [3,0,1]
// Output: 2
//
// Example 2:
//
// Input: [9,6,4,2,3,5,7,0,1]
// Output: 8
//
// Note:
// Your algorithm should run in linear runtime complexity. Could you implement
// it using only constant extra space complexity?

// the question asks us to solve in linear runtime an constant spacetime.
// use the fact that a xor a = 0, but 0 xor a = a, and that the numbers go from
// 0..n if we xor this with all the numbers, we'll get the missing one.
pub fn missing_number(nums: Vec<i32>) -> i32 {
    nums.iter()
        .enumerate()
        .fold(nums.len() as i32, |mut missing, (i, n)| {
            missing ^= (i as i32) ^ n;
            missing
        })
}
