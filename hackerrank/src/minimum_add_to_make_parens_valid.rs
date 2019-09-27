fn min_add_to_make_valid(s: String) -> isize {
    let mut opening = 0;
    let mut bal = 0;
    for c in s.chars() {
        bal += if c == '(' { 1 } else { -1 };
        if bal == -1 {
            bal += 1;
            opening += 1;
        }
    }
    opening + bal
}

#[test]
fn test() {}
