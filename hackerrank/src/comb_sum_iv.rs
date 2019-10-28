#![feature(box_into_raw_non_null)]
// 377. Combination Sum IV
// Medium
// Given an integer array with all positive numbers and no duplicates, find the
// number of possible combinations that add up to a positive integer target.
// Example:
// nums = [1, 2, 3]
// target = 4
// The possible combination ways are:
// (1, 1, 1, 1)
// (1, 1, 2)
// (1, 2, 1)
// (1, 3)
// (2, 1, 1)
// (2, 2)
// (3, 1)
// Note that different sequences are counted as different combinations.
// Therefore the output is 7.

use std::collections::HashMap;

pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
    let mut map = HashMap::new();
    comb_sum(&nums, target, &mut map) as i32
}

pub fn comb_sum(nums: &[i32], target: i32, map: &mut HashMap<i32, usize>) -> usize {
    if target < 0 {
        return 0;
    }
    if target == 0 {
        return 1;
    }
    match map.get(&target) {
        Some(c) => *c,
        None => {
            let mut comb = 0;
            for val in nums {
                comb += comb_sum(nums, target - val, map);
            }
            map.insert(target, comb);
            comb
        }
    }
}
