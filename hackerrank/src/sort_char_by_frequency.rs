// 451. Sort Characters By Frequency
// Medium
// Given a string, sort it in decreasing order based on the frequency of
// characters. Example 1:
// Input:
// "tree"
// Output:
// "eert"
// Explanation:
// 'e' appears twice while 'r' and 't' both appear once.
// So 'e' must appear before both 'r' and 't'. Therefore "eetr" is also a valid
// answer.

pub fn frequency_sort(s: String) -> String {
    use std::{collections::HashMap, iter, str};
    let s = s.as_bytes();
    let map = s.iter().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });

    let mut freq = map.iter().collect::<Vec<_>>();
    freq.sort_by(|(_, v0), (_, v1)| v1.cmp(v0));

    str::from_utf8(
        &freq
            .into_iter()
            .map(|(&k, &v)| iter::repeat(*k).take(v))
            .flatten()
            .collect::<Vec<_>>(),
    )
    .unwrap()
    .to_string()
}
