// 39. Combination Sum
// Medium

// Given a set of candidate numbers (candidates) (without duplicates) and a
// target number (target), find all unique combinations in candidates where the
// candidate numbers sums to target.

// The same repeated number may be chosen from candidates unlimited number of
// times.

// Note:

//     All numbers (including target) will be positive integers.
//     The solution set must not contain duplicate combinations.

// Example 1:

// Input: candidates = [2,3,6,7], target = 7,
// A solution set is:
// [
//   [7],
//   [2,2,3]
// ]

// Example 2:

// Input: candidates = [2,3,5], target = 8,
// A solution set is:
// [
//   [2,2,2,2],
//   [2,3,3],
//   [3,5]
// ]

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut ret = vec![];
    let mut temp = vec![];
    combinations(&candidates, target, 0, &mut ret, &mut temp);
    ret
}

fn combinations(
    candidates: &[i32],
    target: i32,
    start: usize,
    ret: &mut Vec<Vec<i32>>,
    temp: &mut Vec<i32>,
) {
    if target == 0 {
        ret.push(temp.clone());
        return;
    }
    for i in start..candidates.len() {
        if target >= candidates[i] {
            temp.push(candidates[i]);
            combinations(candidates, target - candidates[i], i, ret, temp);
            temp.pop();
        }
    }
}
