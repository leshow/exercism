// 209. Minimum Size Subarray Sum
// Medium

// Given an array of n positive integers and a positive integer s, find the
// minimal length of a contiguous subarray of which the sum ≥ s. If there isn't
// one, return 0 instead.

// Example:

// Input: s = 7, nums = [2,3,1,2,4,3]
// Output: 2
// Explanation: the subarray [4,3] has the minimal length under the problem
// constraint.

pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
    use std::cmp;
    if nums.is_empty() {
        return 0;
    }
    let mut right = 0;
    let mut left = 0;
    let mut len = usize::max_value();
    let mut sum = 0;
    while right < nums.len() {
        sum += nums[right];
        while sum >= s {
            let dist = right + 1 - left;
            len = cmp::min(dist, len);
            sum -= nums[left];
            left += 1;
        }
        right += 1;
    }    cmp::max(0, len as i32)
}
