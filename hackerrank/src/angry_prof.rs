// Enter your code here
use std::{
    error::Error,
    io::{self, BufRead},
};

const YES: &str = "YES";
const NO: &str = "NO";

fn run() -> Result<(), Box<dyn Error>> {
    let stdio = io::stdin();
    let mut lines = stdio.lock().lines();
    let t = lines.next().unwrap().unwrap().parse::<usize>()?;
    for _ in 0..=t {
        let n_v = lines
            .next()
            .unwrap()
            .unwrap()
            .split(' ')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let n = n_v.last().unwrap();
        let arr = lines
            .next()
            .unwrap()
            .unwrap()
            .split(' ')
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        println!("{}", angry_prof(arr, *n));
    }

    Ok(())
}

fn angry_prof(arr: Vec<i32>, threshold: usize) -> &'static str {
    let ontime_students = arr.into_iter().filter(|&t| t <= 0).collect::<Vec<i32>>();
    if ontime_students.len() >= threshold {
        NO
    } else {
        YES
    }
}
