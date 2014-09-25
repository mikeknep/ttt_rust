mod ttt;

fn main() {
    println!("Let's play Tic-Tac-Toe!");

    let mut board: ttt::core::board::Board = ttt::core::board::Board::new();
    let player_1: ttt::core::player::Player = ttt::core::player::Player::new(ttt::core::board::X,
                                                                             ttt::core::unbeatable_ai::choose_best_available_cell);
    let player_2: ttt::core::player::Player = ttt::core::player::Player::new(ttt::core::board::O,
                                                                             ttt::core::unbeatable_ai::choose_best_available_cell);

    while !ttt::core::rules::is_game_over(&board) {
        ttt::console_ui::presenter::display_board(&board);

        let current_player: ttt::core::player::Player =
            ttt::core::rules::current_player(board.cells, player_1, player_2);
        let spot_to_play: uint = current_player.choose_next_move(&board);
        let mutable_cells = board.cells.as_mut_slice();
        let token = Some(current_player.token);

        ttt::core::gameplay_executor::execute_turn(mutable_cells, spot_to_play, token);
    }

    ttt::console_ui::presenter::display_board(&board);
    println!("Game over!");
}
