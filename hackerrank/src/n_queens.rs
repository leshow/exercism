pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let n = n as usize;
    let mut ret = Vec::new();
    let mut board = (0..n)
        .map(|_| (0..n).map(|_| '.').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    solve(&mut board, n, 0, &mut ret);
    ret
}

fn solve(board: &mut Vec<Vec<char>>, n: usize, col: usize, ret: &mut Vec<Vec<String>>) -> bool {
    if col >= n {
        let new_board = board.iter().map(|r| r.iter().collect()).collect::<Vec<_>>();
        ret.push(new_board);
        return true;
    }
    let mut res = false;
    for row in 0..n {
        if safe(board, row, col) {
            board[row][col] = 'Q';
            res = solve(board, n, col + 1, ret) || res;
            board[row][col] = '.';
        }
    }
    res
}

fn safe(board: &[Vec<char>], row: usize, col: usize) -> bool {
    for i in 0..col {
        if board[row][i] == 'Q' {
            return false;
        }
    }
    let mut i = row;
    let mut j = col;
    while i > 0 && j > 0 {
        if board[i][j] == 'Q' {
            return false;
        }
        i -= 1;
        j -= 1;
    }
    i = row;
    j = col;
    while i < board.len() && j > 0 {
        if board[i][j] == 'Q' {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}
#[test]
fn test_safe() {
    let b1 = vec![
        "..Q.".chars().collect(),
        "Q...".chars().collect(),
        ".Q..".chars().collect(),
        "...Q".chars().collect(),
    ];
    // let b3 = vec![
    //     ".Q..".chars().collect(),
    //     "..Q.".chars().collect(),
    //     "Q...".chars().collect(),
    //     "...Q".chars().collect(),
    // ];
    assert!(!safe(&b1, 2, 1));
    // assert!(safe(&b3, 1, 2));
}
