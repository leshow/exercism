#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    error::Error,
    io::{stdout, BufWriter, Write},
};

use crate::scanner::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());
    let n = scan.next::<usize>();
    let m = scan.next::<usize>();
    let mut topics = vec![];
    (0..n).for_each(|_| topics.push(scan.next::<String>()));
    let (total, all) = get_topics(topics, n)?;
    writeln!(out, "{}", total)?;
    writeln!(out, "{}", all)?;
    Ok(())
}

// The solution has topics up to length 500, meaning it produces integer
// 2^500 in size, blowing past 64-bit integer capacity. So, here's a BigInt
// solution:
// ```rust
// use num::{BigInt, Num, Zero};
// fn get_topics(topics: Vec<String>, n: usize, _m: usize) -> Result<(usize, usize), Box<dyn Error>> {
//     let mut max_topics = 0;
//     let mut total_max = 0;
//     for i in 0..(n-1) {
//         for j in (i+1)..n {
//             let one = BigInt::from_str_radix(&topics[i], 2)?;
//             let two = BigInt::from_str_radix(&topics[j], 2)?;
//             let set_bits = count_set_bits(one | two);
//             if set_bits > max_topics {
//                 max_topics = set_bits;
//                 total_max = 1;
//             } else if set_bits == max_topics {
//                 total_max += 1;
//             }
//         }
//     }
//     Ok((max_topics, total_max))
// }
// fn count_set_bits(n: BigInt) -> usize {
//     let mut n = n.clone();
//     let mut count = 0;
//     while n > BigInt::zero() {
//         n &= n.clone()-1;
//         count += 1;
//     }
//     count
// }
// ```
//
fn get_topics(topics: Vec<String>, n: usize) -> Result<(usize, usize), Box<dyn Error>> {
    let mut max_topics = 0;
    let mut total_max = 0;

    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let one = usize::from_str_radix(&topics[i], 2)?;
            let two = usize::from_str_radix(&topics[j], 2)?;
            let set_bits = count_set_bits(one | two);
            if set_bits > max_topics {
                max_topics = set_bits;
                total_max = 1;
            } else if set_bits == max_topics {
                total_max += 1;
            }
        }
    }
    Ok((max_topics, total_max))
}

fn count_set_bits(n: usize) -> usize {
    let mut n = n;
    let mut count = 0;
    while n > 0 {
        n &= n - 1;
        count += 1;
    }
    count
}
