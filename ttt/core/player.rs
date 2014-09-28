use super::board::{Token, Board, X, O};

pub struct Player {
    pub token: Token,
    decision_maker: fn(&Board, Token) -> uint
}

impl Player {
    pub fn new(token: Token, decision_maker: fn(&Board, Token) -> uint) -> Player {
        Player { token: token, decision_maker: decision_maker }
    }

    pub fn new_player_1(decision_maker: fn(&Board, Token) -> uint) -> Player {
        Player { token: X, decision_maker: decision_maker }
    }

    pub fn new_player_2(decision_maker: fn(&Board, Token) -> uint) -> Player {
        Player { token: O, decision_maker: decision_maker }
    }

    pub fn choose_next_move(&self, board: &Board) -> uint {
        let dm = self.decision_maker;
        dm(board, self.token)
    }
}




#[cfg(test)]
mod test {
    use super::Player;
    use super::super::board::{Board, X, O};
    use super::super::test_helpers;


    #[test]
    fn creates_player_1() {
        let player_1: Player = Player::new_player_1(test_helpers::mock_decision_maker);
        assert!(player_1.token == X);
    }

    #[test]
    fn creates_player_2() {
        let player_2: Player = Player::new_player_2(test_helpers::mock_decision_maker);
        assert!(player_2.token == O);
    }

    #[test]
    fn choosing_next_move_returns_cell_index() {
        let player: Player = Player::new(O, test_helpers::mock_decision_maker);
        let board = Board::new();

        assert!(player.choose_next_move(&board) == 4u);
    }
}
