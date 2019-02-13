use rayon::prelude::*;
use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    (1..(sum / 3))
        .into_par_iter()
        .filter_map(|a| {
            let b_plus_c = sum - a;
            let b = (b_plus_c.pow(2) - a.pow(2)) / (b_plus_c * 2);
            let c = b_plus_c - b;
            if a < b && ((a * a) + (b * b) == c * c) {
                Some([a, b, c])
            } else {
                None
            }
        })
        .collect::<HashSet<[u32; 3]>>()
}
