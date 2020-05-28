pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut ret = mat
        .into_iter()
        .enumerate()
        .map(|(i, row)| (i, row.into_iter().sum::<i32>()))
        .collect::<Vec<_>>();
    ret.sort_by(|&(_, a), &(_, b)| a.cmp(&b));

    ret.into_iter()
        .map(|(i, _)| i as i32)
        .take(k as usize)
        .collect::<Vec<_>>()
}
