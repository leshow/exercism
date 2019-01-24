// Enter your code here 
use std::io::{self, BufRead};
use std::collections::HashSet;

fn get_pairs(colors: Vec<usize>, socks: usize) -> usize {
    let mut set = HashSet::new();
    let mut pairs = 0;
    for sock in &colors {
        if !set.contains(sock) {
            set.insert(sock);
        } else {
            pairs += 1;
            set.remove(sock);
        }
    }
    pairs
}

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();

    let socks = iter.next().unwrap().unwrap().parse::<usize>().unwrap();
    let colors = iter.next().unwrap().unwrap().trim_right().split(" ").map(|x| x.parse::<usize>()).collect::<Result<Vec<usize>, _>>().unwrap();
    
    print!("{}", get_pairs(colors, socks));
}

