use std::{
    error::Error,
    io::{self, BufRead, Write},
};

fn jump_clouds(c: &[u8], n: usize) -> usize {
    let mut jumps = 0;
    let mut i = 0;
    while i < c.len() {
        if i < n - 2 && c[i + 2] != 1 {
            i += 1;
        }
        jumps += 1;
        i += 1;
    }
    jumps - 1
}

fn main() -> Result<(), Box<Error>> {
    let stdin = io::stdin();
    let mut stditer = stdin.lock().lines();
    let n = stditer.next().unwrap()?.parse::<usize>()?;
    let clouds = stditer.next().unwrap()?;
    let bin = clouds
        .split(' ')
        .map(|s| s.parse::<_>())
        .collect::<Result<Vec<u8>, _>>()?;

    let jumps = jump_clouds(&bin, n);
    println!("{}", jumps);
    io::stdout().flush();

    Ok(())
}
