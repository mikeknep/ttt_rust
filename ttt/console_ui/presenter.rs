use super::super::core::board::{Board, X, O};

pub fn display_board(board: &Board) {
    for n in range(0, board.cell_count()) {
        let separator = match n % board.length() {
            0 => "\n",
            _ => "|"
        };
        let display_cell = match board.cells[n] {
            Some(X) => "X",
            Some(O) => "O",
            None => " "
        };

        print!("{}{}", separator, display_cell);
    }
    print!("\n");
}
