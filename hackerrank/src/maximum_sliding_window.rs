// 239. Sliding Window Maximum
// Hard

// Given an array nums, there is a sliding window of size k which is moving from
// the very left of the array to the very right. You can only see the k numbers
// in the window. Each time the sliding window moves right by one position.
// Return the max sliding window.

// Example:

// Input: nums = [1,3,-1,-3,5,3,6,7], and k = 3
// Output: [3,3,5,5,6,7]
// Explanation:

// Window position                Max
// ---------------               -----
// [1  3  -1] -3  5  3  6  7       3
//  1 [3  -1  -3] 5  3  6  7       3
//  1  3 [-1  -3  5] 3  6  7       5
//  1  3  -1 [-3  5  3] 6  7       5
//  1  3  -1  -3 [5  3  6] 7       6
//  1  3  -1  -3  5 [3  6  7]      7

// Note:
// You may assume k is always valid, 1 ≤ k ≤ input array's size for non-empty
// array.

// Follow up:
// Could you solve it in linear time?
pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::cmp;
    let k = k as usize;
    let mut ret = Vec::new();
    for i in 0..=(nums.len() - k) {
        ret.push(
            nums[i..(i + k)]
                .iter()
                .fold(i32::min_value(), |x, &y| cmp::max(x, y)),
        );
    }
    ret
}

pub fn max_sliding_window_queue(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::cmp;
    use std::collections::VecDeque;
    if k == 1 || nums.is_empty() {
        return nums;
    }
    let k = k as usize;
    let (mut q, mut max) =
        nums[0..k]
            .iter()
            .fold((VecDeque::new(), i32::min_value()), |(mut q, max), &n| {
                q.push_back(n);
                (q, cmp::max(n, max))
            });
    let mut res = Vec::new();
    for i in k..nums.len() {
        res.push(max);
        if nums[i] > max {
            max = nums[i];
            q = VecDeque::new();
            q.push_back(nums[i]);
        } else if q.len() < k {
            q.push_back(nums[i]);
        } else {
            let el = q.pop_front().unwrap();
            q.push_back(nums[i]);
            if el == max {
                max = q.iter().fold(i32::min_value(), |a, &b| cmp::max(a, b));
            }
        }
    }
    res.push(max);
    res
}
