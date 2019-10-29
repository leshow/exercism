// 771. Jewels and Stones
// Easy
// You're given strings J representing the types of stones that are jewels, and
// S representing the stones you have.  Each character in S is a type of stone
// you have.  You want to know how many of the stones you have are also jewels.
// The letters in J are guaranteed distinct, and all characters in J and S are
// letters. Letters are case sensitive, so "a" is considered a different type of
// stone from "A". Example 1:
// Input: J = "aA", S = "aAAbbbb"
// Output: 3
// Example 2:
// Input: J = "z", S = "ZZ"
// Output: 0

pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
    use std::collections::HashMap;
    let jewels = j.chars().into_iter().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });

    s.chars()
        .filter(|c| jewels.contains_key(c))
        .collect::<Vec<_>>()
        .len() as i32
}
