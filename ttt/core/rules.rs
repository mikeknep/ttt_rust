use super::board::{Board, Token, X, O};
use super::player::Player;

pub fn current_player_and_opponent(cells: &[Option<Token>], p1: Player, p2: Player) -> (Player, Player) {
    let mut played_cells = cells.iter().filter(|cell| cell.is_some());
    match played_cells.count() % 2 {
        0 => (p1, p2),
        _ => (p2, p1)
    }
}

pub fn winning_token(board: &Board) -> Token {
    assert!(is_winner_on_board(board));
    let mut played_cells = board.cells.iter().filter(|cell| cell.is_some());
    match played_cells.count() % 2 {
        0 => O,
        _ => X
    }
}

pub fn is_valid_position(position: uint, board: &Board) -> bool {
    position < board.cell_count() && board.cells[position].is_none()
}

pub fn is_game_over(board: &Board) -> bool {
    is_winner_on_board(board) || is_full(board)
}

pub fn is_winner_on_board(board: &Board) -> bool {
    let all_winning_paths = all_winning_paths(board.length());
    let mut paths = all_winning_paths.iter();
    paths.any(|path| is_winner_on_path(path, board))
}

fn all_winning_paths(board_length: uint) -> Vec<Vec<uint>> {
    let mut paths: Vec<Vec<uint>> = Vec::new();
    for n in range(0, board_length) {
        paths.push(Vec::from_fn(board_length, |idx| board_length * n + idx));
        paths.push(Vec::from_fn(board_length, |idx| board_length * idx + n));
    }
    let (diagonal_1, diagonal_2) = diagonal_indexes(board_length);
    paths.push(diagonal_1);
    paths.push(diagonal_2);

    paths
}

fn diagonal_indexes(board_length: uint) -> (Vec<uint>, Vec<uint>) {
    let mut diagonal_1 = Vec::new();
    let mut diagonal_2 = Vec::new();
    for n in range(0, board_length) {
        diagonal_1.push((board_length - 1) * (n + 1));
        diagonal_2.push((board_length + 1) * (n));
    }
    (diagonal_1, diagonal_2)
}

fn is_winner_on_path(path: &Vec<uint>, board: &Board) -> bool {
    let mut tokens = Vec::with_capacity(board.length());
    for n in range(0, path.iter().count()) {
        let cell_index = path[n];
        tokens.push(board.cells[cell_index]);
    }
    tokens.iter().all(|token| *token == Some(X)) || tokens.iter().all(|token| *token == Some(O))
}

fn is_full(board: &Board) -> bool {
    let mut cells = board.cells.iter();
    !cells.any(|cell| cell.is_none())
}






#[cfg(test)]
mod test {
    use super::{is_valid_position, is_game_over, is_full, is_winner_on_board, is_winner_on_path, current_player_and_opponent, winning_token, all_winning_paths};
    use super::super::board::{Board, X, O};
    use super::super::player::Player;
    use super::super::test_helpers;

    #[test]
    fn recognizes_player_1_as_current_player_on_empty_board() {
        let board: Board = Board::new();
        let p1: Player = Player::new_player_1(test_helpers::mock_decision_maker);
        let p2: Player = Player::new_player_2(test_helpers::mock_decision_maker);
        let (player1, player2) = current_player_and_opponent(board.cells, p1, p2);

        assert!(p1.token == player1.token);
        assert!(p2.token == player2.token);
    }

    #[test]
    fn recognizes_player_2_as_current_player_on_in_progress_board() {
        let board: Board = test_helpers::new_board_with_layout(
            vec!(Some(X), Some(X), Some(O),
                 Some(O), Some(X), Some(O),
                 Some(X), None   , None   ));
        let p1: Player = Player::new_player_1(test_helpers::mock_decision_maker);
        let p2: Player = Player::new_player_2(test_helpers::mock_decision_maker);
        let (player2, player1) = current_player_and_opponent(board.cells, p1, p2);

        assert!(p1.token == player1.token);
        assert!(p2.token == player2.token);
    }

    #[test]
    fn returns_correct_winning_token_x() {
        let board: Board = test_helpers::new_board_with_layout(
            vec!(Some(X), Some(X), Some(X),
                 None   , None   , None   ,
                 Some(O), Some(O), None   ));

        assert!(winning_token(&board) == X);
    }

    #[test]
    fn returns_correct_winning_token_o() {
        let board: Board = test_helpers::new_board_with_layout(
            vec!(Some(X), Some(X), None   ,
                 Some(X), None   , None   ,
                 Some(O), Some(O), Some(O)));

        assert!(winning_token(&board) == O);
    }

    #[test]
    fn index_out_of_bounds_is_invalid() {
        let board: Board = Board::new();
        assert!(is_valid_position(13, &board) == false);
    }

    #[test]
    fn already_played_spot_is_invalid() {
        let mut board: Board = Board::new();
        { board.cells.as_mut_slice()[5] = Some(O); }

        assert!(is_valid_position(5, &board) == false);
    }

    #[test]
    fn negative_index_is_invalid() {
        let board: Board = Board::new();
        assert!(is_valid_position(-1, &board) == false);
    }

    #[test]
    fn spots_on_board_are_valid() {
        let board: Board = Board::new();
        for n in range(0u, board.cell_count()) {
            assert!(is_valid_position(n, &board));
        }
    }

    #[test]
    fn determines_game_over_when_board_full() {
        let board: Board = test_helpers::new_board_with_layout(
            vec!(Some(O), Some(X), Some(X),
                 Some(X), Some(O), Some(O),
                 Some(O), Some(X), Some(X)));

        assert!(is_full(&board));
        assert!(is_game_over(&board));
    }

    #[test]
    fn determines_game_over_when_player_wins() {
        let path = vec!(0,1,2);
        let board: Board = test_helpers::new_board_with_layout(
            vec!(Some(X), Some(X), Some(X),
                 None   , None   , None   ,
                 Some(O), Some(O), None   ));

        assert!(is_winner_on_path(&path, &board));
        assert!(is_winner_on_board(&board));
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

    #[test]
    fn returns_all_winning_paths_on_3_by_3_board() {
        assert_eq!(all_winning_paths(3u), vec!(vec!(0,1,2), vec!(0,3,6), vec!(3,4,5), vec!(1,4,7), vec!(6,7,8), vec!(2,5,8), vec!(2,4,6), vec!(0,4,8)));
    }
}
