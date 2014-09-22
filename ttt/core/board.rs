#[deriving(PartialEq)]
enum Token {
    X, O
}

struct Board {
    cells: [Option<Token>, ..9]
}

impl Board {
    fn new() -> Board {
        Board { cells: [None, ..9] }
    }
}



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
