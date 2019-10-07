// 739. Daily Temperatures
// Medium

// Given a list of daily temperatures T, return a list such that, for each day
// in the input, tells you how many days you would have to wait until a warmer
// temperature. If there is no future day for which this is possible, put 0
// instead.

// For example, given the list of temperatures T = [73, 74, 75, 71, 69, 72, 76,
// 73], your output should be [1, 1, 4, 2, 1, 1, 0, 0].

// Note: The length of temperatures will be in the range [1, 30000]. Each
// temperature will be an integer in the range [30, 100].

pub fn daily_temperatures_naive(t: Vec<i32>) -> Vec<i32> {
    let mut ret = vec![];
    for i in 0..t.len() {
        let start = t[i];
        let mut added = false;
        for (count, j) in ((i + 1)..t.len()).enumerate() {
            if t[j] > start {
                added = true;
                ret.push(count as i32 + 1);
                break;
            }
        }
        if !added {
            ret.push(0 as i32);
        }
    }
    ret
}

pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
    use std::collections::VecDeque;
    let mut stack: VecDeque<usize> = VecDeque::new();
    let mut ret = vec![];
    for i in (0..t.len()).rev() {
        while !stack.is_empty() && t[*stack.back().unwrap()] <= t[i] {
            stack.pop_back();
        }
        ret.push(if stack.is_empty() {
            0
        } else {
            *stack.back().unwrap() - i
        } as i32);
        stack.push_back(i);
    }    ret.into_iter().rev().collect::<Vec<_>>()
}
