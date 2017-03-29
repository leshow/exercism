use std::collections::HashMap;

pub fn word_count(strings: &str) -> HashMap<String, u32> {
    let mut map: HashMap<String, u32> = HashMap::new();

    for sub in strings
            .split(|c: char| !c.is_alphanumeric())
            .filter(|c| c.len() != 0)
            .map(|c| c.to_lowercase()) {
        let count = map.entry(sub.to_owned()).or_insert(0);
        *count += 1;
    }
    map
}
