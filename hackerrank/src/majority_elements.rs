// https://leetcode.com/problems/majority-element/
fn majority_element(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let majority = nums.len() >> 1;
    let map = nums.iter().fold(HashMap::new(), |mut map, num| {
        *map.entry(num).or_insert(0) += 1;
        map
    });
    *map.into_iter().find(|&(_, num)| num > majority).unwrap().0
}
