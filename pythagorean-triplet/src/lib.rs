use rayon::prelude::*;
use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    (1..(sum / 3))
        .into_par_iter()
        .flat_map(|a| {
            (a + 1..(sum / 2))
                .into_par_iter()
                .filter(move |b| {
                    let c = sum - a - b;
                    (a * a) + (b * b) == c * c
                })
                .map(move |b| [a, b, (sum - a - b)])
        })
        .collect::<HashSet<[u32; 3]>>()
}
