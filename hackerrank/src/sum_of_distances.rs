// 834. Sum of Distances in Tree
// Hard
// An undirected, connected tree with N nodes labelled 0...N-1 and N-1 edges are
// given. The ith edge connects nodes edges[i][0] and edges[i][1] together.
// Return a list ans, where ans[i] is the sum of the distances between node i
// and all other nodes. Example 1:
// Input: N = 6, edges = [[0,1],[0,2],[2,3],[2,4],[2,5]]
// Output: [8,12,6,10,10,10]
// Explanation:
// Here is a diagram of the given tree:
//   0
//  / \
// 1   2
//    /|\
//   3 4 5
// We can see that dist(0,1) + dist(0,2) + dist(0,3) + dist(0,4) + dist(0,5)
// equals 1 + 1 + 2 + 2 + 2 = 8.  Hence, answer[0] = 8, and so on.

fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut matrix = vec![vec![]; n];
    for edge in &edges {
        matrix[edge[0] as usize].push(edge[1]);
        matrix[edge[1] as usize].push(edge[0]);
    }
    let mut distmatrix = vec![vec![]; n];
    let mut len = 0;
    for i in 0..n {
        dfs(&matrix, i as i32, -1, &mut distmatrix, &mut len);
    }
    distmatrix
        .into_iter()
        .map(|arr| arr.into_iter().sum())
        .collect::<Vec<_>>()
}

fn dfs(matrix: &[Vec<i32>], i: i32, parent: i32, distmatrix: &mut Vec<Vec<i32>>, len: &mut i32) {
    for j in 0..matrix[i as usize].len() {
        let node = matrix[i as usize][j];
        if node == parent {
            continue;
        }
        *len += 1;
        dfs(matrix, node, i, distmatrix, len);
        *len -= 1;
    }
    distmatrix[i as usize].push(*len)
}

#[test]
fn test_sum() {
    assert_eq!(
        sum_of_distances_in_tree(
            6,
            [[0, 1], [0, 2], [2, 3], [2, 4], [2, 5]]
                .iter()
                .map(|a| a.to_vec())
                .collect::<Vec<_>>()
        ),
        [8, 12, 6, 10, 10, 10].to_vec()
    )
}
