// 794. Valid Tic-Tac-Toe State
// Medium

// A Tic-Tac-Toe board is given as a string array board. Return True if and only
// if it is possible to reach this board position during the course of a valid
// tic-tac-toe game.

// The board is a 3 x 3 array, and consists of characters " ", "X", and "O".
// The " " character represents an empty square.

// Here are the rules of Tic-Tac-Toe:

//     Players take turns placing characters into empty squares (" ").
//     The first player always places "X" characters, while the second player
// always places "O" characters.     "X" and "O" characters are always placed
// into empty squares, never filled ones.     The game ends when there are 3 of
// the same (non-empty) character filling any row, column, or diagonal.
//     The game also ends if all squares are non-empty.
//     No more moves can be played if the game is over.

// Example 1:
// Input: board = ["O  ", "   ", "   "]
// Output: false
// Explanation: The first player always plays "X".

// Example 2:
// Input: board = ["XOX", " X ", "   "]
// Output: false
// Explanation: Players take turns making moves.

// Example 3:
// Input: board = ["XXX", "   ", "OOO"]
// Output: false

// Example 4:
// Input: board = ["XOX", "O O", "XOX"]
// Output: true

pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
    use std::collections::HashMap;
    let board = board
        .into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let moves = board
        .iter()
        .flatten()
        .fold(HashMap::new(), |mut map, &mov| {
            *map.entry(mov).or_insert(0) += 1;
            map
        });
    let count = |x| *(moves.get(&x).unwrap_or(&0));
    let os = count('O');
    let xs = count('X');
    if xs - os > 1 {
        return false;
    }
    if xs < os {
        return false;
    }
    if xs >= 3 || os >= 3 {
        let board = board.iter().map(|row| &row[..]).collect::<Vec<_>>();
        let xwin = win(&board, 'X');
        let owin = win(&board, 'O');
        if xwin && owin {
            return false;
        }
        if xwin {
            return xs - os == 1;
        } else if owin {
            return xs == os;
        }
    }
    true
}

pub fn win(board: &[&[char]], c: char) -> bool {
    for i in 0..board.len() {
        let mut vcount = 0;
        let mut hcount = 0;
        for j in 0..board[0].len() {
            if c == board[i][j] {
                hcount += 1;
            }
            if c == board[j][i] {
                vcount += 1;
            }
        }
        if vcount == board.len() || hcount == board.len() {
            return true;
        }
    }
    let mut dcount = 0;
    let mut rdcount = 0;
    let mut rd = board.len() - 1;
    for (d, i) in (0..board.len()).enumerate() {
        if board[i][d] == c {
            dcount += 1;
        }
        if board[rd][i] == c {
            rdcount += 1;
        }
        rd -= 1;
    }
    if dcount == board.len() || rdcount == board.len() {
        return true;
    }
    false
}
