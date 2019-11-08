// 40. Combination Sum II
// Medium

// Given a collection of candidate numbers (candidates) and a target number
// (target), find all unique combinations in candidates where the candidate
// numbers sums to target.

// Each number in candidates may only be used once in the combination.

// Note:

//     All numbers (including target) will be positive integers.
//     The solution set must not contain duplicate combinations.

// Example 1:

// Input: candidates = [10,1,2,7,6,1,5], target = 8,
// A solution set is:
// [
//   [1, 7],
//   [1, 2, 5],
//   [2, 6],
//   [1, 1, 6]
// ]

// Example 2:

// Input: candidates = [2,5,2,1,2], target = 5,
// A solution set is:
// [
//   [1,2,2],
//   [5]
// ]

pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    use std::collections::HashSet;
    let mut ret = HashSet::new();
    let mut combinations = vec![];
    candidates.sort();

    fn sum_to(
        candidates: &[i32],
        combinations: &mut Vec<i32>,
        ret: &mut HashSet<Vec<i32>>,
        target: i32,
        i: usize,
    ) {
        if target == 0 {
            ret.insert(combinations.clone());
            return;
        } else if i >= candidates.len() || target < 0 {
            return;
        }
        for j in i..candidates.len() {
            combinations.push(candidates[j]);
            sum_to(candidates, combinations, ret, target - candidates[j], j + 1);
            combinations.pop();
        }
    }
    sum_to(&candidates, &mut combinations, &mut ret, target, 0);
    ret.into_iter().collect()
}
