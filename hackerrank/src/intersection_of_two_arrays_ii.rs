// https://leetcode.com/problems/intersection-of-two-arrays-ii/
use std::collections::HashMap;
fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut map = nums1.iter().fold(HashMap::new(), |mut map, num| {
        *map.entry(num).or_insert(0) += 1;
        map
    });

    let mut ret = Vec::new();
    for num in &nums2 {
        map.entry(num).and_modify(|e| {
            if *e > 0 {
                ret.push(*num);
                *e -= 1;
            }
        });
    }
    ret
}
