// this function overflows in single sized arrays. I think because mid-1 where
// mid is usize and 0 overflows to the max available integer
pub fn search(arr: Vec<i32>, elem: i32) -> i32 {
    use std::cmp::Ordering::*;
    let mut r = arr.len() - 1;
    let mut l = 0;

    while l <= r {
        let mid = l + (r - l) / 2;
        match arr[mid].cmp(&elem) {
            Less => {
                l = mid + 1;
            }
            Greater => {
                r = mid - 1;
            }
            Equal => return mid as i32,
        };
    }
    -1
}
