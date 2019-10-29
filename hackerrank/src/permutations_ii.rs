use std::collections::HashSet;

pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut set = HashSet::new();
    let r = nums.len() - 1;
    gen_2(&mut nums, 0, r, &mut set);
    set.into_iter().collect::<Vec<_>>()
}

fn gen_2(nums: &mut [i32], l: usize, r: usize, set: &mut HashSet<Vec<i32>>) {
    if l == r {
        let v = nums.to_vec();
        if !set.contains(&v) {
            set.insert(v);
        }
        return;
    }
    for i in l..=r {
        nums.swap(l, i);
        gen_2(nums, l + 1, r, set);
        nums.swap(l, i);
    }
}
