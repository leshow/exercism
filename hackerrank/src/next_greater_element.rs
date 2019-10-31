// 496. Next Greater Element I
// Easy
// You are given two arrays (without duplicates) nums1 and nums2 where nums1’s
// elements are subset of nums2. Find all the next greater numbers for nums1's
// elements in the corresponding places of nums2. The Next Greater Number of a
// number x in nums1 is the first greater number to its right in nums2. If it
// does not exist, output -1 for this number. Example 1:
// Input: nums1 = [4,1,2], nums2 = [1,3,4,2].
// Output: [-1,3,-1]
// Explanation:
//     For number 4 in the first array, you cannot find the next greater number
// for it in the second array, so output -1.     For number 1 in the first
// array, the next greater number for it in the second array is 3.
//     For number 2 in the first array, there is no next greater number for it
// in the second array, so output -1. Example 2:
// Input: nums1 = [2,4], nums2 = [1,2,3,4].
// Output: [3,-1]
// Explanation:
//     For number 2 in the first array, the next greater number for it in the
// second array is 3.     For number 4 in the first array, there is no next
// greater number for it in the second array, so output -1.

fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut stack = Vec::new();
    let mut map = HashMap::new();
    for num in nums2 {
        // while num is bigger than the last item in our stack, insert it into the map
        while !stack.is_empty() && *stack.last().unwrap() < num {
            map.insert(stack.pop().unwrap(), num);
        }
        stack.push(num);
    }
    // all of these have no element larger, so we put -1
    while let Some(n) = stack.pop() {
        map.insert(n, -1);
    }
    nums1
        .into_iter()
        .filter_map(|n| map.get(&n))
        .copied()
        .collect::<Vec<_>>()
}