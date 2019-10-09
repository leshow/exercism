#[allow(unused_imports)]
use std::{
    error::Error,
    io::{stdin, stdout, BufWriter, Write},
    iter::Iterator,
};

#[derive(Default)]
pub struct Scanner {
    buffer: Vec<String>,
}

impl Scanner {
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
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
