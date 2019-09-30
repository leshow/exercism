use std::{
    collections::hash_map::DefaultHasher,
    hash::{self, Hash, Hasher},
};

pub fn grep(haystack: &str, needle: &str) -> Vec<usize> {
    let mut ret = Vec::new();
    let k = needle.len();
    let haystack = haystack.as_bytes();
    let needle = needle.as_bytes();
    let nhash = hash(needle, 101);
    let mut hhash = hash(&haystack[0..k], 101);
    dbg!(nhash);
    dbg!(hhash);
    let h: u32 = 256u32.pow(k as u32 - 1);
    dbg!(h);
    for i in 0..=(haystack.len() - k) {
        if nhash == hhash {
            // check
            if &haystack[i..i + k] == needle {
                ret.push(i);
            }
        }
        if i < (haystack.len() - k) {
            //remove leftmost char
            hhash -= haystack[i] as u32 * h;
            // shift all chars by 1
            hhash *= 256;
            // add new char to hash
            hhash = (hhash + haystack[i + k] as u32) % 101;
            // hhash = 256 * (hhash - haystack[i] * h);
        }
    }
    ret
}

fn hash(h: &[u8], prime: u32) -> u32 {
    let mut ret = 0;
    // let mut hasher = DefaultHasher::new();
    // <str as Hash>::hash(h, &mut hasher);
    // hasher.finish()
    for b in h {
        ret = ((ret * 256) + *b as u32) % prime;
    }
    ret
}

#[test]
fn test_hash() {
    assert_eq!(grep("GEEKS FOR GEEKS", "GEEKS"), vec![0, 10]);
}
