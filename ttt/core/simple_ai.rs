use super::board::Board;

pub fn choose_first_open_spot(board: &Board) -> uint {
    for n in range(0, 9u) {
        if board.cells[n] == None {
            return n;
        }
    }
    fail!("There are no open spots on the board. This turn should not have been played.");
}





#[cfg(test)]
mod test {
    use super::choose_first_open_spot;
    use super::super::board::{Board, X, O};

    #[test]
    fn chooses_first_open_spot() {
        let mut board: Board = Board::new();
        {
            let cells = board.cells.as_mut_slice();
            cells[0] = Some(X);
            cells[1] = Some(O);
        }

        assert!(choose_first_open_spot(&board) == 2u);
    }

    #[test]
    #[should_fail]
    fn fails_if_no_open_spots() {
        let mut board: Board = Board::new();
        {
            let cells = board.cells.as_mut_slice();
            cells[0] = Some(X);
            cells[1] = Some(X);
            cells[2] = Some(O);
            cells[3] = Some(O);
            cells[4] = Some(O);
            cells[5] = Some(X);
            cells[6] = Some(X);
            cells[7] = Some(X);
            cells[8] = Some(O);
        }
        choose_first_open_spot(&board);
    }
}
