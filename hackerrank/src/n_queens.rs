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
