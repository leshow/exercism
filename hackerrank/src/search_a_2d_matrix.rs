// 74. Search a 2D Matrix
// Medium

// Write an efficient algorithm that searches for a value in an m x n matrix.
// This matrix has the following properties:

//     Integers in each row are sorted from left to right.
//     The first integer of each row is greater than the last integer of the
// previous row.

// Example 1:

// Input:
// matrix = [
//   [1,   3,  5,  7],
//   [10, 11, 16, 20],
//   [23, 30, 34, 50]
// ]
// target = 3
// Output: true

// Example 2:

// Input:
// matrix = [
//   [1,   3,  5,  7],
//   [10, 11, 16, 20],
//   [23, 30, 34, 50]
// ]
// target = 13
// Output: false

pub fn search_matrix(mut matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let sorted = matrix
        .drain(..)
        .flat_map(|v| v.into_iter())
        .collect::<Vec<_>>();
    sorted.binary_search(&target).is_ok()
}
