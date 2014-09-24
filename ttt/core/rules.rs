use super::board::{Board, Token};
use super::player::Player;

pub fn current_player(cells: &[Option<Token>], p1: Player, p2: Player) -> Player {
    let mut played_cells = cells.iter().filter(|&cell| *cell != None);
    match played_cells.count() % 2 {
        0 => p1,
        _ => p2
    }
}

pub fn is_valid_position(position: int, cells: &[Option<Token>]) -> bool {
    position >= 0 && position <= 8 && cells[position as uint] == None
}

pub fn is_game_over(board: &Board) -> bool {
    is_winner_on_board(board) || is_full(board)
}

pub fn is_winner_on_board(board: &Board) -> bool {
    let winning_paths = all_winning_paths();
    let mut iterable_paths = winning_paths.iter();
    iterable_paths.any(|path| is_winner_on_path(path, board))
}

fn all_winning_paths() -> Vec<[uint, ..3]> {
    vec!([0,1,2], [3,4,5], [6,7,8], [0,3,6], [1,4,7], [2,5,8], [0,4,8], [2,4,6])
}

fn is_winner_on_path(path: &[uint, ..3], board: &Board) -> bool {
    let token_1 = board.cells[path[0]];
    let token_2 = board.cells[path[1]];
    let token_3 = board.cells[path[2]];
    token_1 == token_2 && token_2 == token_3 && token_3 != None
}

fn is_full(board: &Board) -> bool {
    let mut cells = board.cells.iter();
    !cells.any(|cell| *cell == None)
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
    fn recognizes_player_1_as_current_player() {
        let board: Board = Board::new();
        let p1: Player = Player::new(X, mock_decision_maker);
        let p2: Player = Player::new(O, mock_decision_maker);

        assert!(current_player(board.cells, p1, p2).eq(&p1));
    }

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
    fn determines_game_over_when_player_wins() {
        let mut board: Board = Board::new();
        {
            let cells = board.cells.as_mut_slice();
            cells[0] = Some(X);
            cells[4] = Some(X);
            cells[8] = Some(X);
        }

        assert!(is_game_over(&board));
    }

    #[test]
    fn determines_game_not_over_when_no_winner() {
        let mut board: Board = Board::new();
        {
            let cells = board.cells.as_mut_slice();
            cells[0] = Some(X);
            cells[1] = Some(O);
        }

        assert!(is_game_over(&board) == false);
    }
}
