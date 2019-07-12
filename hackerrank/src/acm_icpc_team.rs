#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    error::Error,
    io::{stdin, stdout, BufWriter, Write},
};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());
    let n = scan.next::<usize>();
    let m = scan.next::<usize>();
    let mut topics = vec![];
    (0..n).for_each(|_| topics.push(scan.next::<String>()));
    let (total, all) = get_topics(topics, n, m)?;
    writeln!(out, "{}", total)?;
    writeln!(out, "{}", all)?;
    Ok(())
}

fn get_topics(topics: Vec<String>, n: usize, m: usize) -> Result<(usize, usize), Box<dyn Error>> {
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
