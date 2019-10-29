// 46. Permutations
// Medium
// Given a collection of distinct integers, return all possible permutations.
// Example:
// Input: [1,2,3]
// Output:
// [
//   [1,2,3],
//   [1,3,2],
//   [2,1,3],
//   [2,3,1],
//   [3,1,2],
//   [3,2,1]
// ]

pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ret = Vec::new();
    let r = nums.len() - 1;
    gen(&mut nums, 0, r, &mut ret);
    ret
}

fn gen(nums: &mut [i32], l: usize, r: usize, ret: &mut Vec<Vec<i32>>) {
    if l == r {
        ret.push(nums.to_vec());
        return;
    }
    for i in l..=r {
        nums.swap(l, i);
        gen(nums, l + 1, r, ret);
        nums.swap(l, i);
    }
}
