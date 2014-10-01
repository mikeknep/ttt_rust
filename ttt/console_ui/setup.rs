use super::super::core::board::Board;
use super::super::core::player::Player;
use super::io;

pub fn new_game<R: Buffer>(reader: &mut R) -> (Board, Player, Player) {
    let player_1_decision_maker = io::get_player_decision_maker(1u, reader);
    let player_2_decision_maker = io::get_player_decision_maker(2u, reader);

    (Board::new(), Player::new_player_1(player_1_decision_maker), Player::new_player_2(player_2_decision_maker))
}




#[cfg(test)]
mod test {
    use std::io::BufReader;
    use super::new_game;
    use super::super::super::core::board::{X, O};

    #[test]
    fn returns_an_empty_board() {
        let mut reader = BufReader::new(b"e\ne\n");
        let (board, _, _) = new_game(&mut reader);
        let mut cells = board.cells.iter();

        assert!(cells.count() == 9u);
        assert!(cells.all(|cell| cell.is_none()));
    }

    #[test]
    fn returns_players_with_proper_tokens() {
        let mut reader = BufReader::new(b"e\ne\n");
        let (_, player_1, player_2) = new_game(&mut reader);

        assert!(player_1.token == X);
        assert!(player_2.token == O);
    }
}
