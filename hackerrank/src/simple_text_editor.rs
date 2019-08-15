use crate::scanner::*;

use std::io::{self, BufWriter, Write};

#[derive(Clone, Debug)]
enum Op<T> {
    Append { value: T },
    Del { idx: usize },
    Print { idx: usize },
    Undo,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(io::stdout());

    let len = scan.next::<usize>();
    let ops = (0..len)
        .map(|_| {
            let op = scan.next::<usize>();
            match op {
                1 => {
                    let value = scan.next::<String>().into_bytes();
                    Op::Append { value }
                }
                2 => {
                    let idx = scan.next::<usize>();
                    Op::Del { idx }
                }
                3 => {
                    let idx = scan.next::<usize>();
                    Op::Print { idx }
                }
                4 => Op::Undo,
                _ => panic!("unknown op"),
            }
        })
        .collect::<Vec<_>>();
    edit(ops, out);
    Ok(())
}

fn edit<W>(ops: Vec<Op<Vec<u8>>>, out: &mut BufWriter<W>)
where
    W: Write,
{
    let mut s: Vec<u8> = Vec::new();
    let mut stack: Vec<Vec<u8>> = Vec::new();
    for op in ops {
        match op {
            Op::Append { value } => {
                stack.push(s.clone());
                s.extend(&value);
            }
            Op::Del { idx } => {
                stack.push(s.clone());
                (0..idx).for_each(|_| {
                    let _ = s.pop();
                });
            }
            Op::Print { idx } => {
                writeln!(out, "{}", s[idx - 1] as char);
            }
            Op::Undo => {
                if let Some(last_s) = stack.pop() {
                    s = last_s;
                }
            }
        }
    }
    s
}
