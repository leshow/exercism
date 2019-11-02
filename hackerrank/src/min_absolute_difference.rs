// 1200. Minimum Absolute Difference
// Easy
// Given an array of distinct integers arr, find all pairs of elements with the
// minimum absolute difference of any two elements. Return a list of pairs in
// ascending order(with respect to pairs), each pair [a, b] follows     a, b are
// from arr     a < b
//     b - a equals to the minimum absolute difference of any two elements in
// arr Example 1:
// Input: arr = [4,2,1,3]
// Output: [[1,2],[2,3],[3,4]]
// Explanation: The minimum absolute difference is 1. List all pairs with
// difference equal to 1 in ascending order. Example 2:
// Input: arr = [1,3,6,10,15]
// Output: [[1,3]]
// Example 3:
// Input: arr = [3,8,-10,23,19,-4,-14,27]
// Output: [[-14,-10],[19,23],[23,27]]

// 16ms 4.4mb
pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
    use std::cmp;
    arr.sort();
    let mut min_diff = i32::max_value();
    for i in 1..arr.len() {
        min_diff = cmp::min(arr[i] - arr[i - 1], min_diff);
    }
    let mut ret = Vec::new();
    for i in 1..arr.len() {
        if arr[i] - arr[i - 1] == min_diff {
            ret.push(vec![arr[i - 1], arr[i]]);
        }
    }
    ret
}

// 12ms 4.3mb
// Iterator wins!
pub fn minimum_abs_difference_2(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
    use std::cmp;
    arr.sort();
    let min = arr[..].windows(2).fold(i32::max_value(), |min, slice| {
        cmp::min(slice[1] - slice[0], min)
    });
    arr.windows(2)
        .filter_map(|slice| {
            if slice[1] - slice[0] == min {
                Some(vec![slice[0], slice[1]])
            } else {
                None
            }
        })
        .collect::<Vec<Vec<_>>>()
}
