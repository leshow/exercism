use std::{cmp::Reverse, collections::BinaryHeap};
/// Write a class RecentCounter to count recent requests.
///
/// It has only one method: ping(int t), where t represents some time in
/// milliseconds.
///
/// Return the number of pings that have been made from 3000 milliseconds ago
/// until now.
///
/// Any ping with time in [t - 3000, t] will count, including the current ping.
///
/// It is guaranteed that every call to ping uses a strictly larger value of t
/// than before.
///
///  
///
/// Example 1:
///
/// Input: inputs = ["RecentCounter","ping","ping","ping","ping"], inputs =
/// [[],[1],[100],[3001],[3002]] Output: [null,1,2,3,3]
///
///  
///
/// Note:
///
///     Each test case will have at most 10000 calls to ping.
///     Each test case will call ping with strictly increasing values of t.
///     Each call to ping will have 1 <= t <= 10^9.
struct RecentCounter {
    heap: BinaryHeap<Reverse<i32>>,
}

impl RecentCounter {
    fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    // why the return type is i32 here I don't know
    fn ping(&mut self, t: i32) -> i32 {
        self.heap.push(Reverse(t));
        while let Some(Reverse(n)) = self.heap.peek() {
            if *n < t - 3000 {
                self.heap.pop();
            } else {
                break;
            }
        }
        self.heap.len() as i32
    }
}
