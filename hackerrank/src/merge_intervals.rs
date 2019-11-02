// 56. Merge Intervals
// Medium

// Given a collection of intervals, merge all overlapping intervals.

// Example 1:

// Input: [[1,3],[2,6],[8,10],[15,18]]
// Output: [[1,6],[8,10],[15,18]]
// Explanation: Since intervals [1,3] and [2,6] overlaps, merge them into [1,6].

// Example 2:

// Input: [[1,4],[4,5]]
// Output: [[1,5]]
// Explanation: Intervals [1,4] and [4,5] are considered overlapping.

// NOTE: input types have been changed on April 15, 2019. Please reset to
// default code definition to get new method signature.

pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::cmp;
    let mut ret: Vec<Vec<i32>> = Vec::new();
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    for win in intervals {
        match ret.last_mut() {
            Some(last) if last[1] < win[0] => ret.push(win),
            Some(last) => last[1] = cmp::max(last[1], win[1]),
            None => ret.push(win),
        }
    }
    ret
}
