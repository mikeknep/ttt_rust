use std::num::Int;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread::Thread;
use super::board::{Board, Token};
use super::rules;

pub fn choose_best_available_cell(board: &Board, tokens: (Token, Token)) -> usize {
    let open_cells = collect_open_cells(board);
    let (tx, rx): (Sender<usize>, Receiver<usize>) = mpsc::channel();
    for &cell_index in open_cells.iter() {
        let task_tx = tx.clone();
        let board_ = *board;
        Thread::spawn(move|| {
            let cell_score = score_cell(cell_index, &board_, tokens, 1);
            task_tx.send((cell_index, cell_score));
        });
    }
    let open_cell_count = open_cells.iter().count();
    let mut scores: Vec<(usize, f64)> = Vec::with_capacity(open_cell_count);
    for _ in range(0, open_cell_count) {
        scores.push(rx.recv());
    }
    get_highest_scored_cell(scores)
}

fn score_cell(cell_index: usize, board: &Board, tokens: (Token, Token), depth: usize) -> f64 {
    let mut board = *board;
    {
        let cells = board.cells.as_mut_slice();
        super::gameplay_executor::execute_turn(cells, cell_index, tokens.val0());
    }

    if rules::is_game_over(&board) {
        score_board(&board, depth)
    } else {
        let mut comp_score = Int::pow(-1f64, depth + 1us);
        let new_open_cells = collect_open_cells(&board);
        for &cell_index in new_open_cells.iter() {
            let cell_score = score_cell(cell_index, &board, swap_tokens(tokens), depth + 1us);
            if should_update_score(cell_score, comp_score, depth) {
                comp_score = cell_score
            }
        }
        comp_score
    }
}

fn should_update_score(cell_score: f64, comp_score: f64, depth: usize) -> bool {
    (depth % 2 == 0 && cell_score > comp_score) || (depth % 2 == 1 && cell_score < comp_score)
}

fn swap_tokens(tokens: (Token, Token)) -> (Token, Token) {
    let (current_token, opponent_token) = tokens;
    (opponent_token, current_token)
}

fn collect_open_cells(board: &Board) -> Vec<usize> {
    let mut open_cells = vec![];
    for n in range(0, board.cell_count()) {
        if board.cells[n].is_none() {
            open_cells.push(n);
        }
    }
    open_cells
}

fn get_highest_scored_cell(scores: Vec<(usize, f64)>) -> usize {
    let mut best_cell_and_score = (0us, -2f64);
    for &cell_and_score in scores.iter() {
        if cell_and_score.val1() > best_cell_and_score.val1() {
            best_cell_and_score = cell_and_score;
        }
    }
    best_cell_and_score.val0()
}

fn score_board(board: &Board, depth: usize) -> f64 {
    if rules::is_winner_on_board(board) {
        Int::pow(-1f64, depth + 1us) / depth as f64
    } else {
        0f64
    }
}





#[cfg(test)]
mod test {
    use super::{choose_best_available_cell, score_board, get_highest_scored_cell, collect_open_cells, should_update_score};
    use super::super::board::{Board};
    use super::super::board::Token::{X, O};
    use super::super::test_helpers;


    #[test]
    fn chooses_last_open_spot_in_cats_game() {
        let board: Board = test_helpers::new_board_with_layout(
            vec!(None   , Some(X), Some(O),
                 Some(O), Some(X), Some(X),
                 Some(X), Some(O), Some(O)));

        assert_eq!(choose_best_available_cell(&board, (X,O)), 0us);
    }

    #[test]
    fn chooses_last_open_spot_in_victory() {
        let board: Board = test_helpers::new_board_with_layout(
            vec!(Some(X), Some(O), Some(X),
                 Some(O), Some(X), Some(X),
                 Some(O), Some(X), None   ));

        assert_eq!(choose_best_available_cell(&board, (X,O)), 8us);
    }

    #[test]
    fn chooses_immediate_winning_spot_1() {
        let board: Board = test_helpers::new_board_with_layout(
            vec!(Some(X), None   , Some(O),
                 Some(O), Some(X), None   ,
                 Some(X), Some(O), None   ));

        assert_eq!(choose_best_available_cell(&board, (X,O)), 8us);
    }

    #[test]
    fn chooses_immediate_winning_spot_2() {
        let board: Board = test_helpers::new_board_with_layout(
            vec!(Some(X), None   , Some(O),
                 Some(O), Some(X), Some(O),
                 Some(X), Some(X), None   ));

        assert_eq!(choose_best_available_cell(&board, (O,X)), 8us);
    }

    #[test]
    fn chooses_move_that_would_lead_to_opponent_victory_1() {
        let board: Board = test_helpers::new_board_with_layout(
            vec!(Some(X), Some(O), Some(X),
                 Some(X), Some(O), None   ,
                 Some(O), None   , None   ));

        assert_eq!(choose_best_available_cell(&board, (X,O)), 7us);
    }

    #[test]
    fn chooses_move_that_would_lead_to_opponent_victory_2() {
        let board: Board = test_helpers::new_board_with_layout(
            vec!(Some(O), Some(X), None   ,
                 None   , Some(X), None   ,
                 None   , None   , None   ));

        assert_eq!(choose_best_available_cell(&board, (O,X)), 7us);
    }

    #[test]
    fn returns_indexes_of_open_cells() {
        let board: Board = test_helpers::new_board_with_layout(
            vec!(Some(X), Some(X), None   ,
                 None   , Some(O), None   ,
                 None   , None   , None   ));

        assert!(collect_open_cells(&board) == vec![2, 3, 5, 6, 7, 8])
    }

    #[test]
    fn returns_index_of_highest_scored_cell() {
        let scores = vec![(1us, 0.25f64), (2us, 1f64), (4us, 0f64), (8us, -0.5f64)];
        assert_eq!(get_highest_scored_cell(scores), 2us);
    }

    #[test]
    fn scores_board_with_no_winner_as_0() {
        let board: Board = Board::new();
        assert!(score_board(&board, 1) == 0f64);
    }

    #[test]
    fn scores_draw_board_as_0() {
        let board: Board = test_helpers::new_board_with_layout(
            vec!(Some(X), Some(X), Some(O),
                 Some(O), Some(O), Some(X),
                 Some(X), Some(X), Some(O)));

        assert!(score_board(&board, 1) == 0f64);
    }

    #[test]
    fn scores_board_with_winner_at_1_depth_positive_1() {
        let board: Board = test_helpers::new_board_with_layout(
            vec!(Some(X), Some(X), Some(X),
                 None   , None   , None   ,
                 None   , None   , None   ));

        assert_eq!(score_board(&board, 1), 1f64);
    }


    #[test]
    fn scores_board_with_winner_at_depth_2_as_negative_0_5() {
        let board: Board = test_helpers::new_board_with_layout(
            vec!(Some(X), Some(X), Some(X),
                 None   , None   , None   ,
                 None   , None   , None   ));

        assert_eq!(score_board(&board, 2), -0.5f64);
    }

    #[test]
    fn should_update_score_when_even_depth_and_cell_greater_than_comp() {
        assert!(should_update_score(1f64, -0.5f64, 2us));
    }

    #[test]
    fn should_update_score_when_odd_depth_and_cell_less_than_comp() {
        assert!(should_update_score(-0.25f64, 0.2f64, 3us));
    }
}
