///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits. However, your function must be able to
///     process input with leading 0 digits.
///
use std::collections::VecDeque;

type Num = u32;

#[allow(unused_variables)]
pub fn convert<T: AsRef<[Num]>>(number: T, from_base: Num, to_base: Num) -> Result<Vec<Num>, ()> {
    let mut num = number
        .as_ref()
        .iter()
        .rev()
        .enumerate()
        .map(|(i, a)| a * from_base.pow(i as Num))
        .sum::<Num>();

    let mut res = VecDeque::new();
    while num >= 0 {
        let r = num % to_base;
        res.push_front(r);
        num = num / to_base;
    }
    Ok(res.into())
}
