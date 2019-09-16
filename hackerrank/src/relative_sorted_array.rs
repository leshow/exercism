// 1122. Relative Sort Array
// Easy

// Given two arrays arr1 and arr2, the elements of arr2 are distinct, and all
// elements in arr2 are also in arr1.

// Sort the elements of arr1 such that the relative ordering of items in arr1
// are the same as in arr2.  Elements that don't appear in arr2 should be placed
// at the end of arr1 in ascending order.

// Example 1:

// Input: arr1 = [2,3,1,3,2,4,6,7,9,2,19], arr2 = [2,1,4,3,9,6]
// Output: [2,2,2,1,4,3,3,9,6,7,19]

pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;

    let mut map = arr1.iter().fold(HashMap::new(), |mut map, num| {
        *map.entry(num).or_insert(0) += 1;
        map
    });
    let mut ret = vec![];
    for num in &arr2 {
        map.entry(num).and_modify(|e| {
            if *e > 0 {
                for _ in 0..*e {
                    ret.push(*num);
                    *e -= 1;
                }
            }
        });
    }
    let mut additional = vec![];
    for (k, v) in &map {
        if *v != 0 {
            for _ in 0..*v {
                additional.push(*k);
            }
        }
    }
    additional.sort();
    ret.extend(additional);
    ret
}

pub fn relative_sort_array_bounds(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut counts = [0; 1001];
    for &num in &arr1 {
        counts[num as usize] += 1;
    }

    let mut ret = vec![];
    for &num in &arr2 {
        let c_ref = &mut counts[num as usize];
        if *c_ref > 0 {
            ret.extend((0..*c_ref).map(|_| {
                *c_ref -= 1;
                num
            }));
        }
    }
    counts
        .iter()
        .enumerate()
        .filter(|(_, &n)| n != 0)
        .for_each(|(i, &n)| {
            let i = i as i32;
            ret.extend((0..n).map(|_| i))
        });

    ret
}
