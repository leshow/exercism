/// leetcode
pub fn min_deletion_size(a: Vec<String>) -> i32 {
    if a.is_empty() {
        return 0;
    }
    let mut min = 0;
    let a = a.iter().map(|n| n.as_bytes()).collect::<Vec<_>>();
    for i in 0..a[0].len() {
        for j in 1..a.len() {
            if a[j][i] < a[j - 1][i] {
                min += 1;
                break;
            }
        }
    }
    min
}

// [1,2] [3,4] [5,6]
