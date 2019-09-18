// 992. Subarrays with K Different Integers
// Hard
// Given an array A of positive integers, call a (contiguous, not necessarily
// distinct) subarray of A good if the number of different integers in that
// subarray is exactly K. (For example, [1,2,3,1,2] has 3 different integers: 1,
// 2, and 3.) Return the number of good subarrays of A.
// Example 1:
// Input: A = [1,2,1,2,3], K = 2
// Output: 7
// Explanation: Subarrays formed with exactly 2 different integers: [1,2],
// [2,1], [1,2], [2,3], [1,2,1], [2,1,2], [1,2,1,2]. Example 2:
// Input: A = [1,2,1,3,4], K = 3
// Output: 3
// Explanation: Subarrays formed with exactly 3 different integers: [1,2,1,3],
// [2,1,3], [1,3,4].

// (i looked at the solution)
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

struct Window<T> {
    counter: HashMap<T, usize>,
    nonzero: usize,
}

impl<T> Window<T> {
    pub fn new() -> Self
    where
        T: Hash + Eq,
    {
        Self {
            counter: HashMap::new(),
            nonzero: 0,
        }
    }

    pub fn add(&mut self, x: T)
    where
        T: Hash + Eq + Copy,
    {
        *self.counter.entry(x).or_insert(0) += 1;
        if self.counter[&x] == 1 {
            self.nonzero += 1;
        }
    }

    pub fn del(&mut self, x: T)
    where
        T: Hash + Eq + Copy,
    {
        self.counter.entry(x).and_modify(|count| *count -= 1);
        if self.counter[&x] == 0 {
            self.nonzero -= 1;
        }
    }

    pub fn nonzero(&self) -> usize {
        self.nonzero
    }
}

// left2 - left1 represents a count of the number of elements between the two
// windows where you keep the "nonzero" counter at exactly K
// that's why they remove ones where you're above K essentially
pub fn subarrays_with_k_distinct(arr: Vec<i32>, k: i32) -> i32 {
    let mut w1: Window<i32> = Window::new();
    let mut w2: Window<i32> = Window::new();

    let mut total = 0;
    let mut left1 = 0;
    let mut left2 = 0;
    let k = k as usize;

    for i in 0..arr.len() {
        w1.add(arr[i]);
        w2.add(arr[i]);
        while w1.nonzero() > k {
            w1.del(arr[left1]);
            left1 += 1;
        }
        while w2.nonzero() >= k {
            w2.del(arr[left2]);
            left2 += 1;
        }
        // equivalent to :
        // for j in left1..left2 {
        // total += 1;
        //}
        total += left2 - left1;
    }
    total as i32
}

// If your nonzero count > k, then you are counting "how many elements can I
// remove before I reach k"

// and for each of those elements, you would have another good subarray

pub fn naive(arr: Vec<i32>, k: usize) -> usize {
    let mut ret = vec![];
    for i in 0..arr.len() {
        for j in 1..=(arr.len() - i) {
            let subarray = arr[i..(i + j)].to_vec();
            if subarray.iter().collect::<HashSet<_>>().len() == k {
                ret.push(subarray);
            }
        }
    }
    ret.len()
}
