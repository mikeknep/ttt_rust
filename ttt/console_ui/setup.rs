use super::super::core::board::Board;
use super::super::core::player::Player;
use super::io;

pub fn new_game() -> (Board, Player, Player) {
    let player_1_decision_maker = io::get_player_decision_maker(1u);
    let player_2_decision_maker = io::get_player_decision_maker(2u);

    (Board::new(), Player::new_player_1(player_1_decision_maker), Player::new_player_2(player_2_decision_maker))
}
