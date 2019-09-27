/// 868. Binary Gap
/// Easy
/// Given a positive integer N, find and return the longest distance between two consecutive 1's in the binary representation of N.
/// If there aren't two consecutive 1's, return 0.
use std::cmp;

// the way leetcode wants it 15 aka 0b1111 returns 1, I don't think this is correct
fn binary_gap(n: i32) -> i32 {
    let mut max = 0;
    let mut n = n;
    let mut count = 0;
    while n > 0 {
        if n & 1 == 1 {
            if count > 0 {
                max = cmp::max(max, count);
            }
            count = 1;
        } else if count > 0 {
            count += 1;
        }
        n >>= 1;
    }
    max
}

#[test]
fn test_gap() {
    assert_eq!(binary_gap(0b1000001), 5);
    assert_eq!(binary_gap(0b100000), 0);
}

// with this we always start at a 1
fn binary_proper(n: i32) -> i32 {
    let mut max = 0;
    let mut gap = 0;
    let mut n = n;
    // remove starting zeroes
    while n > 0 && n & 1 == 0 {
        n >>= 1;
    }
    while n > 0 {
        if n & 1 == 0 {
            gap += 1;
        } else if gap != 0 {
            max = cmp::max(gap, max);
            gap = 0;
        }
        n >>= 1;
    }
    max
}

#[test]
fn test_proper() {
    assert_eq!(binary_proper(0b0001010010), 2)
}
