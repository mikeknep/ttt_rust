use super::board::Token;

pub fn execute_turn(cells: &mut[Option<Token>], cell_index: uint, token: Option<Token>) {
    cells[cell_index] = token;
}



#[cfg(test)]
mod test {
    use super::execute_turn;
    use super::super::board::X;

    #[test]
    fn adds_token_to_board_cells() {
        let mut cells = [None, ..9];
        execute_turn(cells, 4u, Some(X));

        assert!(cells[4] == Some(X));
    }
}
