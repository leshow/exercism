use std::collections::HashMap;

type Res<T> = Result<T, &'static str>;

pub fn count(c: char, nucleotide: &str) -> Res<usize> {
    if !match_base(&c) {
        return Err("Not a valid base");
    }
    let mut matches = 0;
    for ref x in nucleotide.chars() {
        if !match_base(x) {
            return Err("Not a valid base");
        }
        if *x == c {
            matches += 1;
        }
    }
    Ok(matches)
}

fn match_base(b: &char) -> bool {
    match *b {
        'A' | 'T' | 'G' | 'C' => true,
        _ => false,
    }
}

pub fn nucleotide_counts(nucleotide: &str) -> Res<HashMap<char, usize>> {
    let mut counts: HashMap<char, usize> = HashMap::new();
    for base in &['A', 'T', 'G', 'C'] {
        counts.insert(*base, 0);
    }
    for ref x in nucleotide.chars() {
        if !match_base(x) {
            return Err("Not a valid base");
        }
        if let Some(count) = counts.get_mut(x) {
            *count += 1;
        }
    }
    Ok(counts)
}
