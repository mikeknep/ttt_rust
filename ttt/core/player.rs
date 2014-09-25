use super::board::{Token, Board};

pub struct Player {
    pub token: Token,
    decision_maker: fn(&Board, Token) -> uint
}

impl Player {
    pub fn new(token: Token, decision_maker: fn(&Board, Token) -> uint) -> Player {
        Player { token: token, decision_maker: decision_maker }
    }

    pub fn choose_next_move(&self, board: &Board) -> uint {
        let dm = self.decision_maker;
        dm(board, self.token)
    }

    pub fn eq(&self, other: &Player) -> bool {
        self.token == other.token
    }
}




#[cfg(test)]
mod test {
    use super::Player;
    use super::super::board::{Board, Token, O};

    #[allow(unused_variable)]
    fn mock_decision_maker(board: &Board, token: Token) -> uint {
        4u
    }

    #[test]
    fn choosing_next_move_returns_cell_index() {
        let player: Player = Player::new(O, mock_decision_maker);
        let board = Board::new();

        assert!(player.choose_next_move(&board) == 4u);
    }
}
