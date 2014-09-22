use super::board::{Token};

fn execute_turn(cells: &mut[Option<Token>], cell_index: uint, token: Option<Token>) {
    cells[cell_index] = token;
}



#[test]
fn adds_token_to_board_cells() {
    use super::board::{X};
    let mut cells = [None, ..9];
    execute_turn(cells, 4u, Some(X));

    assert!(cells[4] == Some(X));
}
