// 77. Combinations
// Medium

// Given two integers n and k, return all possible combinations of k numbers out
// of 1 ... n.

// Example:

// Input: n = 4, k = 2
// Output:
// [
//   [2,4],
//   [3,4],
//   [2,3],
//   [1,2],
//   [1,3],
//   [1,4],
// ]
pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut ret = Vec::new();
    fn rec(i: i32, n: i32, k: i32, ret: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if k == 0 {
            res.push(ret.clone());
            return;
        }
        for j in i..=n {
            ret.push(j);
            rec(j + 1, n, k - 1, ret, res);
            ret.pop();
        }
    }
    rec(1, n, k, &mut ret, &mut res);
    res
}
