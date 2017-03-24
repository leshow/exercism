use std::error::Error;

pub fn lsp(s: &str, length: usize) -> Result<u32, Box<Error>> {
    if length == 0 {
        return Ok(1);
    }
    if s.chars().any(|c| !c.is_digit(10)) {
        return Err(From::from("Must be a digit"));
    }
    let nums = s.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>();
    if length > nums.len() {
        return Err(From::from("Span too big"));
    }
    // get all contiguous sequences of length
    let mut substrings = Vec::new();
    for i in 0..(nums.len() - length + 1) {
        substrings.push(&nums[i..(i + length)]);
    }
    Ok(substrings.iter()
           .map(|w| w.iter().fold(1, |a, b| a * b))
           .max()
           .unwrap())

    // `windows()` will do this for us
    // Ok(nums.windows(length)
    //        .map(|w| w.iter().fold(1, |a, b| a * b))
    //        .max()
    //        .unwrap())
}
