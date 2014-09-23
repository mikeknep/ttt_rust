use super::board::{Board, Token};

fn is_valid_position(position: int, cells: &[Option<Token>]) -> bool {
    position >= 0 && position <= 8 && cells[position as uint] == None
}

fn is_game_over(board: &Board) -> bool {
    is_winner_on_board(board) || is_full(board)
}

#[allow(unused_variable)]
fn is_winner_on_board(board: &Board) -> bool {
    false
}

fn is_full(board: &Board) -> bool {
    let mut cells = board.cells.iter();
    !cells.any(|cell| *cell == None)
}






#[test]
fn determines_game_over_when_board_full() {
    use super::board::{X};
    let mut board: Board = Board::new();
    {
        let cells = board.cells.as_mut_slice();
        for n in range(0, 9u) {
            cells[n] = Some(X);
        }
    }

    assert!(is_game_over(&board));
}

#[test]
fn determines_game_over_when_player_wins() {}

#[test]
fn index_out_of_bounds_is_invalid() {
    let mut cells = [None, ..9];
    assert!(is_valid_position(13, cells) == false);
}

#[test]
fn already_played_spot_is_invalid() {
    use super::board::{O};
    let mut board: Board = Board::new();
    { board.cells.as_mut_slice()[5] = Some(O); }

    assert!(is_valid_position(5, board.cells.as_slice()) == false);
}
