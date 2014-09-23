use super::board::{Token, Board};

pub struct Player {
    pub token: Token,
    decision_maker: fn(&Board) -> uint
}

impl Player {
    pub fn new(token: Token, decision_maker: fn(&Board) -> uint) -> Player {
        Player { token: token, decision_maker: decision_maker }
    }

    pub fn choose_next_move(&self, board: &Board) -> uint {
        let dm = self.decision_maker;
        dm(board)
    }

    pub fn eq(&self, other: &Player) -> bool {
        self.token == other.token
    }
}





#[test]
fn choosing_next_move_returns_cell_index() {
    use super::board::{O};
    #[allow(unused_variable)]
    fn mock_decision_maker(board: &Board) -> uint {
        4u
    }
    let player: Player = Player::new(O, mock_decision_maker);
    let board = Board::new();

    assert!(player.choose_next_move(&board) == 4u);
}
