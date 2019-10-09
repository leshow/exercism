// 18. 4Sum
// Medium

// Given an array nums of n integers and an integer target, are there elements
// a, b, c, and d in nums such that a + b + c + d = target? Find all unique
// quadruplets in the array which gives the sum of target.

// Note:

// The solution set must not contain duplicate quadruplets.

// Example:

// Given array nums = [1, 0, -1, 0, -2, 2], and target = 0.

// A solution set is:
// [
//   [-1,  0, 0, 1],
//   [-2, -1, 1, 2],
//   [-2,  0, 0, 2]
// ]

// it's slow but it works:
pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    use std::collections::HashSet;
    if nums.len() < 4 {
        return Vec::new();
    }
    let mut ret = HashSet::new();
    for i in 0..(nums.len() - 3) {
        for j in (i + 1)..(nums.len() - 2) {
            for k in (j + 1)..(nums.len() - 1) {
                for t in (k + 1)..nums.len() {
                    if nums[i] + nums[j] + nums[k] + nums[t] == target {
                        let mut ins = vec![nums[i], nums[j], nums[k], nums[t]];
                        ins.sort();
                        ret.insert(ins);
                    }
                }
            }
        }
    }
    ret.drain().collect::<Vec<_>>()
}
