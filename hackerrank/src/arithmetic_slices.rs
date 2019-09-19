// 413. Arithmetic Slices
// Medium
// A sequence of number is called arithmetic if it consists of at least three elements and if the difference between any two consecutive elements is the same.
//
// For example, these are arithmetic sequence:
//
// 1, 3, 5, 7, 9
// 7, 7, 7, 7
// 3, -1, -5, -9
//
// The following sequence is not arithmetic.
//
// 1, 1, 2, 5, 7
//
// A zero-indexed array A consisting of N numbers is given. A slice of that array is any pair of integers (P, Q) such that 0 <= P < Q < N.
//
// A slice (P, Q) of array A is called arithmetic if the sequence:
// A[P], A[p + 1], ..., A[Q - 1], A[Q] is arithmetic. In particular, this means that P + 1 < Q.
//
// The function should return the number of arithmetic slices in the array A.
//
// Example:
//
// A = [1, 2, 3, 4]
//
// return: 3, for 3 arithmetic slices in A: [1, 2, 3], [2, 3, 4] and [1, 2, 3, 4] itself.

pub fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
    let mut count = 0;
    for i in 0..a.len() {
        for j in 3..=(a.len() - i) {
            let diff = a[i + 1] - a[i];
            let mut m = i + 1;
            while m < (i + j) {
                if a[m] - a[m - 1] != diff {
                    break;
                }
                m += 1;
            }
            if m == (i + j) {
                count += 1;
            }
        }
    }
    count
}
