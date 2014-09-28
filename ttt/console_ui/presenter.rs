use super::super::core::board::Board;
use super::super::core::rules;

pub fn display_board(board: &Board) {
    for n in range(0, board.cell_count()) {
        let separator = match n % board.length() {
            0 => "\n",
            _ => "|"
        };

        let cell = board.cells[n];
        let display_cell = match cell.is_some()  {
            true => format!("{}", cell.unwrap()),
            false => String::from_str(" ")
        };

        print!("{}{}", separator, display_cell);
    }
    print!("\n");
}

pub fn present_outcome(board: &Board) {
    display_board(board);
    if rules::is_winner_on_board(board) {
        println!("{} wins!", rules::winning_token(board));
    } else {
        println!("Cat's game!");
    }
}
