use super::board::{Board, Token};

pub fn new_board_with_layout(cell_layout: Vec<Option<Token>>) -> Board {
    let mut board: Board = Board::new();
    let count: uint = board.cell_count();
    {
        let cells = board.cells.as_mut_slice();
        for n in range(0, count) {
            cells[n] = cell_layout[n];
        }
    }
    board
}

pub fn mock_decision_maker(_board: &Board, _tokens: (Token,Token)) -> uint {
    4u
}
