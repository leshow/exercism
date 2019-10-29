// 692. Top K Frequent Words
// Medium
// Given a non-empty list of words, return the k most frequent elements.
// Your answer should be sorted by frequency from highest to lowest. If two
// words have the same frequency, then the word with the lower alphabetical
// order comes first. Example 1:
// Input: ["i", "love", "leetcode", "i", "love", "coding"], k = 2
// Output: ["i", "love"]
// Explanation: "i" and "love" are the two most frequent words.
//     Note that "i" comes before "love" due to a lower alphabetical order.
// Example 2:
// Input: ["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is",
// "is"], k = 4 Output: ["the", "is", "sunny", "day"]
// Explanation: "the", "is", "sunny" and "day" are the four most frequent words,
//     with the number of occurrence being 4, 3, 2 and 1 respectively.

// This solution is not actually accepted because the sorting of strings is off
// but I can't get it to work
pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
    use std::{
        cmp::Reverse,
        collections::{BinaryHeap, HashMap},
    };
    let k = k as usize;
    let map = words.into_iter().fold(HashMap::new(), |mut map, num| {
        *map.entry(num).or_insert(0) += 1;
        map
    });
    map.into_iter()
        .fold(BinaryHeap::new(), |mut heap, (key, v)| {
            heap.push(Reverse(Word::new(key, v)));
            if heap.len() > k {
                heap.pop();
            }
            heap
        })
        .into_iter()
        .map(|count| count.0.s)
        .collect::<Vec<_>>()
}

use std::cmp::Ordering;

#[derive(Eq, Debug)]
struct Word {
    s: String,
    count: usize,
}
impl Word {
    fn new(s: String, count: usize) -> Self {
        Word { s, count }
    }
}

impl Ord for Word {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
    }
}

impl PartialOrd for Word {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.count.cmp(&other.count))
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}
