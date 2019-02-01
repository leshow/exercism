// For example, if we run 9119 through the function, 811181 will come out,
// because 92 is 81 and 12 is 1.

// Note: The function accepts an integer and returns an integer
fn square_digits(mut num: u32) -> u32 {
    let mut res = 0;
    while num != 0 {
        let m = num % 10;
        num /= 10;
        let sq = m.pow(2);
        let digits = (res as f32).log(10 as f32).floor() as u32;
        let ten = 10_u32.pow(1 + digits);
        res += sq * ten;
    }
    res / 10
}
#[cfg(test)]
mod test {
    #[test]
    fn test_square() {
        assert_eq!(super::square_digits(9119), 811_181);
    }
    #[test]
    fn test_again() {
        assert_eq!(super::square_digits(9113), 81119);
    }
}
