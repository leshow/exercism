// broken
pub fn count_of_atoms(formula: String) -> String {
    use std::{collections::HashMap, str};
    let bytes = formula.as_bytes();
    let mut tokens: Vec<HashMap<Vec<u8>, usize>> = vec![];
    let mut cur_map = HashMap::new();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'(' => {
                tokens.push(cur_map);
                cur_map = HashMap::new();
                i += 1;
            }
            b')' => {
                let tmp = cur_map.clone();
                cur_map = tokens.pop().unwrap();
                i += 1;
                let mut num = 1;
                let mut token = vec![];
                while i < bytes.len() && bytes[i].is_ascii_digit() {
                    token.push(bytes[i]);
                    i += 1;
                }
                if !token.is_empty() {
                    num = str::from_utf8(&token).unwrap().parse::<usize>().unwrap();
                }
                for atom in tmp.keys() {
                    *cur_map.entry(atom.to_vec()).or_insert(0) += tmp.get(atom).unwrap_or(&0) * num;
                }
            }
            _ => {
                let mut token = vec![];
                while i < bytes.len() && bytes[i].is_ascii_lowercase() {
                    token.push(bytes[i]);
                    i += 1;
                }

                let mut token = vec![];
                let mut num = 1;
                while i < bytes.len() && bytes[i].is_ascii_digit() {
                    token.push(bytes[i]);
                    i += 1;
                }
                if !token.is_empty() {
                    num = str::from_utf8(&token).unwrap().parse::<usize>().unwrap();
                }
                *cur_map.entry(token).or_insert(0) += num;
            }
        }
    }
    let mut ret = String::new();

    for (atom, count) in cur_map.iter() {
        ret.push_str(str::from_utf8(atom).unwrap());
        if *count > 1 {
            ret.push_str(&format!("{}", count));
        }
    }
    ret
}
