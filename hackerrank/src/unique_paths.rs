fn unique_paths_with_obstacles(grid: Vec<Vec<i32>>) -> i32 {
    let row_len = grid[0].len();

    let mut path = vec![vec![0; row_len + 1]; grid.len() + 1];
    path[0][0] = 1;

    for i in 1..grid.len() {
        if grid[i][0] == 1 {
            path[i][0] = 0;
        } else {
            path[i][0] = path[i - 1][0];
        }
    }

    for j in 1..row_len {
        if grid[0][j] == 1 {
            path[0][j] = 0;
        } else {
            path[0][j] = path[0][j - 1];
        }
    }

    for i in 1..grid.len() {
        for j in 1..row_len {
            if grid[i][j] == 1 {
                path[i][j] = 0;
            } else {
                path[i][j] = path[i - 1][j] + path[i][j - 1];
            }
        }
    }
    path[grid.len() - 1][row_len - 1]
}
