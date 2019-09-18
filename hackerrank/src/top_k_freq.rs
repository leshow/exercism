// 347. Top K Frequent Elements
// Medium

// Given a non-empty array of integers, return the k most frequent elements.

// Example 1:

// Input: nums = [1,1,1,2,2,3], k = 2
// Output: [1,2]

// Example 2:

// Input: nums = [1], k = 1
// Output: [1]

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let k = k as usize;
    let map = nums.into_iter().fold(HashMap::new(), |mut map, num| {
        *map.entry(num).or_insert(0) += 1;
        map
    });
    let mut freq = map.into_iter().collect::<Vec<_>>();
    freq.sort_by(|(_, v0), (_, v1)| v1.cmp(v0));

    freq.into_iter().map(|(k, _)| k).take(k).collect::<Vec<_>>()
}

pub fn top_k_frequent_heap(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::{
        cmp::Reverse,
        collections::{BinaryHeap, HashMap},
    };
    let k = k as usize;

    let map = nums.into_iter().fold(HashMap::new(), |mut map, num| {
        *map.entry(num).or_insert(0) += 1;
        map
    });
    map.into_iter()
        .fold(BinaryHeap::new(), |mut heap, (key, v)| {
            heap.push(Reverse(Count { key, v }));
            if heap.len() > k {
                heap.pop();
            }
            heap
        })
        .into_iter()
        .map(|count| count.0.key)
        .collect::<Vec<_>>()
}

struct Count<T> {
    key: T,
    v: usize,
}
use std::cmp::Ordering;

impl<T> Ord for Count<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.v.cmp(&other.v)
    }
}
impl<T> PartialOrd for Count<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.v.cmp(&other.v))
    }
}

impl<T> PartialEq for Count<T> {
    fn eq(&self, other: &Self) -> bool {
        self.v.eq(&other.v)
    }
}

impl<T> Eq for Count<T> {}
