const BOARD_SIZE: usize = 9us;

#[deriving(PartialEq, Show)]
pub enum Token {
    X, O
}

pub struct Board {
    pub cells: [Option<Token>; BOARD_SIZE]
}

impl Board {
    pub fn new() -> Board {
        let cells = [None; BOARD_SIZE];
        Board { cells: cells }
    }

    pub fn length(&self) -> usize {
        let size = BOARD_SIZE as f64;
        size.sqrt() as usize
    }

    pub fn cell_count(&self) -> usize {
        BOARD_SIZE
    }
}



#[cfg(test)]
mod test {
    use super::Board;

    #[test]
    fn builds_board_with_all_cells_empty() {
        let board: Board = Board::new();
        let mut cells = board.cells.iter();

        assert!(cells.all(|cell| cell.is_none()));
    }

    #[test]
    fn builds_board_with_nine_cells() {
        let board: Board = Board::new();
        let mut cells = board.cells.iter();

        assert!(cells.count() == 9us);
    }
}
