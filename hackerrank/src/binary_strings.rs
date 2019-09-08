pub fn add_binary(a: String, b: String) -> String {
    let a = a.chars().rev().collect::<Vec<char>>();
    let b = b.chars().rev().collect::<Vec<char>>();

    let mut res = Vec::with_capacity(std::cmp::max(a.len(), b.len()));
    let mut carry = false;
    let mut i = 0;

    loop {
        if i >= a.len() && i >= b.len() && !carry {
            break;
        }
        let ax = if i < a.len() { a[i] } else { '0' };
        let bx = if i < b.len() { b[i] } else { '0' };
        match (ax, bx) {
            ('1', '1') => {
                res.push(if carry { '1' } else { '0' });
                carry = true;
            }
            ('0', '0') => {
                res.push(if carry { '1' } else { '0' });
                carry = false;
            }
            _ => {
                res.push(if carry { '0' } else { '1' });
            }
        }
        i += 1;
    }
    if carry {
        res.push('1');
    }
    res.reverse();
    res.into_iter().collect()
}
