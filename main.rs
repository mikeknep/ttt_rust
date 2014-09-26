mod ttt;

fn main() {
    println!("Let's play Tic-Tac-Toe!");
    let (mut board, player_1, player_2) = ttt::core::setup::new_game();
    ttt::console_ui::runner::play_game(&mut board, player_1, player_2);
    ttt::console_ui::presenter::display_board(&board);
    println!("Game over!");
}
