// 599. Minimum Index Sum of Two Lists
// Easy

// Suppose Andy and Doris want to choose a restaurant for dinner, and they both
// have a list of favorite restaurants represented by strings.

// You need to help them find out their common interest with the least list
// index sum. If there is a choice tie between answers, output all of them with
// no order requirement. You could assume there always exists an answer.

// Example 1:

// Input:
// ["Shogun", "Tapioca Express", "Burger King", "KFC"]
// ["Piatti", "The Grill at Torrey Pines", "Hungry Hunter Steakhouse", "Shogun"]
// Output: ["Shogun"]
// Explanation: The only restaurant they both like is "Shogun".

// Example 2:

// Input:
// ["Shogun", "Tapioca Express", "Burger King", "KFC"]
// ["KFC", "Shogun", "Burger King"]
// Output: ["Shogun"]
// Explanation: The restaurant they both like and have the least index sum is
// "Shogun" with index sum 1 (0+1).

#[derive(Eq)]
pub struct Found<'a> {
    pub found: bool,
    pub index: usize,
    pub entry: &'a str,
}
use std::cmp::Ordering;

impl<'a> Ord for Found<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl<'a> PartialOrd for Found<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for Found<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<'a> Found<'a> {
    pub fn new(entry: &'a str, index: usize) -> Self {
        Found {
            entry,
            found: false,
            index,
        }
    }

    pub fn add_index(&mut self, index: usize) {
        self.found = true;
        self.index += index;
    }
}

pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for (i, item) in list1.iter().enumerate() {
        map.entry(item).or_insert_with(|| Found::new(item, i));
    }

    for (i, item) in list2.iter().enumerate() {
        map.entry(item).and_modify(|f| f.add_index(i));
    }
    let mut ret = vec![];
    let mut min = None;
    for (_, entry) in map.iter().filter(|(_, e)| e.found) {
        match min {
            None => {
                min = Some(entry);
            }
            Some(ref e) if entry.index < e.index => {
                min = Some(entry);
            }
            _ => {}
        }
    }
    if let Some(m) = min {
        ret.extend(
            map.iter()
                .filter(|(_, e)| e.found && e.index == m.index)
                .map(|(_, e)| e.entry.into()),
        );
    }
    ret
}
