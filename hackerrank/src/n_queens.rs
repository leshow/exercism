pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    // initial board
    let mut board: Vec<Vec<char>> = (0..n).map(|_| (0..n).map(|_| '.').collect()).collect();
    // return value
    let mut ret = Vec::new();
    // backtracing recursively
    fn rec(board: &mut [Vec<char>], ret: &mut Vec<Vec<String>>, col: usize) -> bool {
        let n = board.len();
        if col >= n {
            let board = board.iter().map(|row| row.iter().collect()).collect();
            ret.push(board);
            return true;
        }
        let mut res = false;
        for i in 0..n {
            if safe(&board, i, col) {
                board[i][col] = 'Q';
                res = rec(board, ret, col + 1) || res;
                board[i][col] = '.';
            }
        }
        res
    }

    // determine if board is safe
    fn safe(board: &[Vec<char>], i: usize, j: usize) -> bool {
        let n = board.len();
        for y in 0..j {
            if board[i][y] != '.' {
                return false;
            }
        }
        for (x, y) in (0..=i).rev().zip((0..=j).rev()) {
            if board[x][y] != '.' {
                return false;
            }
        }
        for (x, y) in (i..n).zip((0..=j).rev()) {
            if board[x][y] != '.' {
                return false;
            }
        }
        true
    }

    if !rec(&mut board, &mut ret, 0) {
        return vec![];
    }
    ret
}

#[test]
fn test_queens() {
    let ret = solve_n_queens(4);
    let res = vec![
        vec![
            "..Q.".to_string(),
            "Q...".to_string(),
            "...Q".to_string(),
            ".Q..".to_string(),
        ],
        vec![
            ".Q..".to_string(),
            "...Q".to_string(),
            "Q...".to_string(),
            "..Q.".to_string(),
        ],
    ];
    assert!(ret.eq(&res));
}
