pub fn find_duplicate(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    for i in 1..nums.len() {
        if nums[i - 1] == nums[i] {
            return nums[i];
        }
    }
    -1
}
