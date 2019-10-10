// 383. Ransom Note
// Easy

// Given an arbitrary ransom note string and another string containing letters
// from all the magazines, write a function that will return true if the ransom
// note can be constructed from the magazines ; otherwise, it will return false.

// Each letter in the magazine string can only be used once in your ransom note.

// Note:
// You may assume that both strings contain only lowercase letters.

// canConstruct("a", "b") -> false
// canConstruct("aa", "ab") -> false
// canConstruct("aa", "aab") -> true

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    use std::collections::HashMap;
    let mut mag_map = magazine.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });
    for c in ransom_note.chars() {
        if (mag_map.contains_key(&c) && mag_map[&c] <= 0) || !mag_map.contains_key(&c) {
            return false;
        }
        mag_map.entry(c).and_modify(|c| *c -= 1);
    }
    true
}
