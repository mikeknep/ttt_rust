use super::board::{Board, Token};
use super::player::Player;

pub fn is_valid_position(position: int, cells: &[Option<Token>]) -> bool {
    position >= 0 && position <= 8 && cells[position as uint] == None
}

pub fn is_game_over(board: &Board) -> bool {
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


pub fn current_player(cells: &[Option<Token>], p1: Player, p2: Player) -> Player {
    let mut played_cells = cells.iter().filter(|&cell| *cell != None);
    match played_cells.count() % 2 {
        0 => p1,
        _ => p2
    }
}





#[cfg(test)]
mod test {
    use super::{is_valid_position, is_game_over, current_player};
    use super::super::board::{Board, X, O};
    use super::super::player::Player;

    #[allow(unused_variable)]
    fn mock_decision_maker(board: &Board) -> uint {
        4u
    }

    #[test]
    fn determines_game_over_when_board_full() {
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
        let cells = [None, ..9];
        assert!(is_valid_position(13, cells) == false);
    }

    #[test]
    fn already_played_spot_is_invalid() {
        let mut board: Board = Board::new();
        { board.cells.as_mut_slice()[5] = Some(O); }

        assert!(is_valid_position(5, board.cells.as_slice()) == false);
    }

    #[test]
    fn recognizes_player_1_as_current_player() {
        let board: Board = Board::new();
        let p1: Player = Player::new(X, mock_decision_maker);
        let p2: Player = Player::new(O, mock_decision_maker);

        assert!(current_player(board.cells, p1, p2).eq(&p1));
    }
}
