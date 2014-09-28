use super::board::{Board, Token};

pub fn choose_first_available_cell(board: &Board, _token: Token) -> uint {
    for n in range(0, board.cell_count()) {
        if board.cells[n].is_none() {
            return n;
        }
    }
    fail!("There are no open spots on the board. This turn should not have been played.");
}





#[cfg(test)]
mod test {
    use super::choose_first_available_cell;
    use super::super::board::{Board, X, O};

    #[test]
    fn chooses_first_available_cell() {
        let mut board: Board = Board::new();
        {
            let cells = board.cells.as_mut_slice();
            cells[0] = Some(X);
            cells[1] = Some(O);
        }

        assert!(choose_first_available_cell(&board, X) == 2u);
    }

    #[test]
    #[should_fail]
    fn fails_if_no_available_cell() {
        let mut board: Board = Board::new();
        let count = board.cell_count();
        {
            let cells = board.cells.as_mut_slice();
            for n in range(0, count) {
                cells[n] = Some(X);
            }
        }
        choose_first_available_cell(&board, O);
    }
}
