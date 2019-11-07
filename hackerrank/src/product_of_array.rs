// 238. Product of Array Except Self
// Medium
// Given an array nums of n integers where n > 1,  return an array output such
// that output[i] is equal to the product of all the elements of nums except
// nums[i]. Example:
// Input:  [1,2,3,4]
// Output: [24,12,8,6]
// Note: Please solve it without division and in O(n).
// Follow up:
// Could you solve it with constant space complexity? (The output array does not
// count as extra space for the purpose of space complexity analysis.)

fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut nums: Vec<(i32, i32)> = nums.into_iter().map(|n| (n, 0)).collect::<Vec<_>>();
    for i in 0..nums.len() {
        nums[i] = (
            nums[i].0,
            nums.iter()
                .map(|(n, _)| *n)
                .enumerate()
                .fold(1, |m, (j, n)| if j == i { m } else { n * m }),
        );
    }
    nums.into_iter().map(|(_, n)| n).collect::<Vec<_>>()
}

fn product_except_self_div(mut nums: Vec<i32>) -> Vec<i32> {
    let product: i32 = nums.iter().product();
    for num in nums.iter_mut() {
        *num = product / *num;
    }
    nums
}

#[test]
fn test_product() {
    assert_eq!(
        product_except_self([1, 2, 3, 4].to_vec()),
        [24, 12, 8, 6].to_vec()
    );
}
