// 581. Shortest Unsorted Continuous Subarray
// Easy

// Given an integer array, you need to find one continuous subarray that if you
// only sort this subarray in ascending order, then the whole array will be
// sorted in ascending order, too.

// You need to find the shortest such subarray and output its length.

// Example 1:

// Input: [2, 6, 4, 8, 10, 9, 15]
// Output: 5
// Explanation: You need to sort [6, 4, 8, 10, 9] in ascending order to make the
// whole array sorted in ascending order.

pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
    use std::cmp;
    let mut sorted = nums.clone();
    sorted.sort();
    let mut start = nums.len();
    let mut end = 0;
    for (i, n) in nums.into_iter().enumerate() {
        if n != sorted[i] {
            start = cmp::min(i, start);
            end = cmp::max(i, end);
        }
    }
    if start < end {
        (end - start + 1) as i32
    } else {
        0
    }
}
