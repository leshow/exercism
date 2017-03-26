use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    // for &(score, ref list) in input.iter() {
    //     for word in list {
    //         res.insert(word.to_owned(), score);
    //     }
    // }
    // res
    // we can do better:
    input.iter()
        .flat_map(|(score, list)| list.iter().map(move |word| (word.to_lowercase(), *score)))
        .collect::<BTreeMap<_, _>>()
}
