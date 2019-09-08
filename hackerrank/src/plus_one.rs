pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    digits.reverse();
    let mut i = 0;
    let mut carry = false;
    loop {
        let digit = &mut digits[i];
        if carry || i == 0 {
            *digit += 1;
            if *digit >= 10 {
                *digit = 0;
                carry = true;
            } else {
                carry = false;
            }
        }
        i += 1;
        if i >= digits.len() {
            break;
        }
    }
    digits.reverse();
    if carry {
        let mut new_digits = vec![1];
        new_digits.extend(&digits);
        digits = new_digits;
    }
    digits
}
