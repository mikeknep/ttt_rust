use super::super::core::board::Board;
use super::super::core::player::Player;
use super::super::core::{rules, gameplay_executor, setup};
use super::{presenter, io};

pub fn start_playing_session() {
    println!("Let's play Tic-Tac-Toe!");
    let mut play_game = true;
    while play_game {
        let (mut board, player_1, player_2) = setup::new_game();
        run_game(&mut board, player_1, player_2);
        presenter::present_outcome(&board);
        play_game = io::get_play_again();
    }
}

fn run_game(board: &mut Board, player_1: Player, player_2: Player) {
    while !rules::is_game_over(board) {
        presenter::display_board(board);

        let current_player: Player = rules::current_player(board.cells, player_1, player_2);
        let spot_to_play: uint = current_player.choose_next_move(board);
        let mutable_cells = board.cells.as_mut_slice();
        let token = Some(current_player.token);

        gameplay_executor::execute_turn(mutable_cells, spot_to_play, token);
    }
}
