pub fn uncommon_from_sentences(a: String, b: String) -> Vec<String> {
    use std::collections::HashMap;
    let mut words_a = a
        .split(' ')
        .chain(b.split(' '))
        .fold(HashMap::new(), |mut map, word| {
            *map.entry(word.clone()).or_insert(0) += 1;
            map
        });
    words_a.retain(|word, count| *count <= 1);
    words_a.keys().map(|s| s.to_string()).collect::<Vec<_>>()
}
