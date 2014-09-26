use super::board::Board;
use super::player::Player;
use super::super::console_ui::io::get_player_decision_maker;

pub fn new_game() -> (Board, Player, Player) {
    let player_1_decision_maker = get_player_decision_maker(1u);
    let player_2_decision_maker = get_player_decision_maker(2u);

    (Board::new(), Player::new_player_1(player_1_decision_maker), Player::new_player_2(player_2_decision_maker))
}
