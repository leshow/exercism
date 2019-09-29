pub fn duplicate_zeros(arr: &mut Vec<i32>) {
    let mut i = 0;
    while i < arr.len() {
        if arr[i] == 0 {
            arr.insert(i, 0);
            i += 1;
            arr.pop();
        }
        i += 1;
    }
}
