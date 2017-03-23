use std::collections::BTreeSet;

pub fn sum_of_flat(n: i64, factors: &[i64]) -> i64 {
    factors.iter()
        .flat_map(|f| (1..((n - 1) / f) + 1).map(move |x| x * f))
        .collect::<BTreeSet<_>>()
        .iter()
        .sum()
}

pub fn sum_of_multiples(n: i64, factors: &mut [i64]) -> i64 {
    fn gcd(a: i64, b: i64) -> i64 {
        if b > 0 { gcd(b, a % b) } else { a }
    };
    factors.sort();
    factors.to_vec().dedup();

    if factors.len() < 1 {
        return 0;
    }

    let product = factors.iter().fold(1, |acc, a| acc * a);

    (1..n).filter(|i| gcd(*i, product) >= factors[0]).sum()
}

pub fn sum_of_filter(n: i64, factors: &[i64]) -> i64 {
    (0..n).filter(|x| factors.iter().any(|a| x % a == 0)).sum()
}