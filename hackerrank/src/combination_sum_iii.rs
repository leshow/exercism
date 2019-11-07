// 216. Combination Sum III
// Medium

// Find all possible combinations of k numbers that add up to a number n, given
// that only numbers from 1 to 9 can be used and each combination should be a
// unique set of numbers.

// Note:

//     All numbers will be positive integers.
//     The solution set must not contain duplicate combinations.

// Example 1:

// Input: k = 3, n = 7
// Output: [[1,2,4]]

// Example 2:

// Input: k = 3, n = 9
// Output: [[1,2,6], [1,3,5], [2,3,4]]

fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    let mut ret = Vec::new();
    go(k, n, vec![], &mut ret);
    ret
}

fn go(k: i32, n: i32, nums: Vec<i32>, ret: &mut Vec<Vec<i32>>) {
    if n == 0 && k == 0 {
        ret.push(nums);
        return;
    }
    if n < 0 || k < 0 {
        return;
    }
    let start = if nums.is_empty() {
        1
    } else {
        nums.last().unwrap() + 1
    };
    for i in start..=9 {
        let mut xs = nums.clone();
        xs.push(i);
        go(k - 1, n - i, xs, ret);
    }
}

// var combinationSum3 = function(k, n) {
//     let ret = [];
//     function go(k, n, nums = []) {
//         if (n == 0 && k == 0) {
//             return ret.push(nums);
//         }
//         if (n < 0 || k < 0) {
//             return;
//         }
//         let start = nums.length == 0 ? 1 : nums[nums.length-1]+1;
//         for (let i = start; i <= 9; i++) {
//             go(k-1, n-i, nums.concat(i));
//         }
//     }
//     go(k, n);
//     return ret;
// };
