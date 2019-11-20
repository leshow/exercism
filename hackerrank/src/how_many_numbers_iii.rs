// We want to generate all the numbers of three digits where:
//     the sum of their digits is equal to 10.
//     their digits are in increasing order (the numbers may have two or more
// equal contiguous digits)
// The numbers that fulfill the two above constraints are: 118, 127, 136, 145,
// 226, 235, 244, 334
// Make a function that receives two arguments:
//     the sum of digits value
//     the desired number of digits for the numbers
// The function should output an array with three values: [1,2,3]
// 1 - the total number of possible numbers
// 2 - the minimum number
// 3 - the maximum number
// The example given above should be:
// findAll 10 3 -> ( 8, Just 118, Just 334 )
// https://www.codewars.com/kata/how-many-numbers-iii/haskell

use std::cmp;

fn gen_three(sum: i32, n: i32, start: i32, nums: &mut Vec<i32>, ret: &mut Vec<i32>) {
    if n == 0 && sum == 0 {
        ret.push(
            nums.clone()
                .into_iter()
                .flat_map(|d| std::char::from_digit(d as u32, 10))
                .collect::<String>()
                .parse::<i32>()
                .unwrap(),
        );
        return;
    }
    if n < 0 || sum < 0 {
        return;
    }
    for i in start..=9 {
        nums.push(i);
        gen_three(sum - i, n - 1, i, nums, ret);
        nums.pop();
    }
}

fn find_all(sum: i32, n: i32) -> (usize, Option<i32>, Option<i32>) {
    let mut ret = vec![];
    let mut nums = vec![];
    gen_three(sum, n, 1, &mut nums, &mut ret);
    if ret.is_empty() {
        (0, None, None)
    } else {
        (
            ret.len(),
            Some(
                ret.iter()
                    .fold(i32::max_value(), |min, &v| cmp::min(min, v)),
            ),
            Some(
                ret.iter()
                    .fold(i32::min_value(), |max, &v| cmp::max(max, v)),
            ),
        )
    }
}
#[test]
fn test_gen() {
    let mut ret = vec![];
    let mut nums = vec![];
    gen_three(10, 3, 1, &mut nums, &mut ret);
    for num in &[118, 127, 136, 145, 226, 235, 244, 334] {
        assert!(ret.contains(num));
    }
}
