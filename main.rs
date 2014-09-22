mod ttt;

fn main() {
    println!("Hello world! Let's play Tic-Tac-Toe!");
    let board: ttt::core::board::Board = ttt::core::board::Board::new();
    ttt::console_ui::presenter::display_board(&board);
}
