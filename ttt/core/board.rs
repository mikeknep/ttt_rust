static BOARD_SIZE: uint = 9u;

#[deriving(PartialEq)]
pub enum Token {
    X, O
}

pub struct Board {
    pub cells: [Option<Token>, ..BOARD_SIZE]
}

impl Board {
    pub fn new() -> Board {
        Board { cells: [None, ..BOARD_SIZE] }
    }

    pub fn length(&self) -> uint {
        let size = BOARD_SIZE as f64;
        size.sqrt() as uint
    }

    pub fn cell_count(&self) -> uint {
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

        assert!(cells.all(|cell| *cell == None));
    }

    #[test]
    fn builds_board_with_nine_cells() {
        let board: Board = Board::new();
        let mut cells = board.cells.iter();

        assert!(cells.count() == 9u);
    }
}
