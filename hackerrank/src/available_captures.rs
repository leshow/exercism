/// 999. Available Captures for Rook
/// Easy
/// On an 8 x 8 chessboard, there is one white rook.  There also may be empty
/// squares, white bishops, and black pawns.  These are given as characters 'R',
/// '.', 'B', and 'p' respectively. Uppercase characters represent white pieces,
/// and lowercase characters represent black pieces. The rook moves as in the
/// rules of Chess: it chooses one of four cardinal directions (north, east,
/// west, and south), then moves in that direction until it chooses to stop,
/// reaches the edge of the board, or captures an opposite colored pawn by
/// moving to the same square it occupies.  Also, rooks cannot move into the
/// same square as other friendly bishops. Return the number of pawns the rook
/// can capture in one move. Input: [
/// [".",".",".",".",".",".",".","."],
/// [".",".",".","p",".",".",".","."],
/// [".",".",".","R",".",".",".","p"],
/// [".",".",".",".",".",".",".","."],
/// [".",".",".",".",".",".",".","."],
/// [".",".",".","p",".",".",".","."],
/// [".",".",".",".",".",".",".","."],
/// [".",".",".",".",".",".",".","."]
/// ]
/// Output: 3
/// Explanation:
/// In this example the rook is able to capture all the pawns.
pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
    let mut r_pos = None;
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] == 'R' {
                r_pos = Some((i, j));
            }
        }
    }
    match r_pos {
        Some((i, j)) => {
            let mut count = 0;
            for n in (0..i).rev() {
                if board[n][j] == 'B' {
                    break;
                } else if board[n][j] == 'p' {
                    count += 1;
                    break;
                }
            }
            for n in i..board.len() {
                if board[n][j] == 'B' {
                    break;
                } else if board[n][j] == 'p' {
                    count += 1;
                    break;
                }
            }
            for n in (0..j).rev() {
                if board[i][n] == 'B' {
                    break;
                } else if board[i][n] == 'p' {
                    count += 1;
                    break;
                }
            }
            for n in j..board[i].len() {
                if board[i][n] == 'B' {
                    break;
                } else if board[i][n] == 'p' {
                    count += 1;
                    break;
                }
            }
            count
        }
        None => 0,
    }
}
