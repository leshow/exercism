// 442. Find All Duplicates in an Array
// Medium
// Given an array of integers, 1 ≤ a[i] ≤ n (n = size of array), some elements
// appear twice and others appear once. Find all the elements that appear twice
// in this array. Could you do it without extra space and in O(n) runtime?
// Example:
// Input:
// [4,3,2,7,8,2,3,1]
// Output:
// [2,3]
use std::collections::HashMap;

pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
    nums.iter()
        .fold(HashMap::new(), |mut map, num| {
            *map.entry(num).or_insert(0) += 1;
            map
        })
        .into_iter()
        .filter_map(|(k, v)| Some(*k).filter(|_| v == 2))
        .collect::<Vec<_>>()
}

enum Form {
    Atom(String, usize),
    Comb(Vec<Form>, usize),
}

fn combine(f: &Form, mut map: HashMap<String, usize>) -> HashMap<String, usize> {
    match f {
        Form::Atom(s, c) => {
            map.entry(s.clone()).or_insert(*c);
            map
        }
        Form::Comb(formulas, c) => {
            let maps = formulas
                .iter()
                .map(|f| combine(f, map.clone()))
                .collect::<Vec<HashMap<_, _>>>();
            for m in maps {
                for (k, v) in m {
                    map.entry(k).and_modify(|b| *b *= c * v);
                }
            }
            map
        }
    }
}
