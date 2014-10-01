use super::super::core::board::Board;
use super::super::core::player::Player;
use super::super::core::{rules, gameplay_executor};
use super::{presenter, io, setup};

pub fn start_playing_session<R: Buffer>(reader: &mut R) {
    println!("Let's play Tic-Tac-Toe!");
    let mut play_game = true;
    while play_game {
        let (mut board, player_1, player_2) = setup::new_game(reader);
        run_game(&mut board, player_1, player_2);
        presenter::present_outcome(&board);
        play_game = io::get_play_again(reader);
    }
    println!("Thanks for playing!");
}

fn run_game(board: &mut Board, player_1: Player, player_2: Player) {
    while !rules::is_game_over(board) {
        presenter::present_board(board);

        let (current_player, opponent): (Player, Player) = rules::current_player_and_opponent(board.cells, player_1, player_2);
        let chosen_cell: uint = current_player.choose_next_move(board, (current_player.token, opponent.token));
        let cells = board.cells.as_mut_slice();
        let token = current_player.token;

        gameplay_executor::execute_turn(cells, chosen_cell, token);
    }
}



#[cfg(test)]
mod test {
    use std::io::BufReader;
    use super::{run_game, start_playing_session};
    use super::super::setup;
    use super::super::super::core::rules;

    #[test]
    fn runs_a_complete_game() {
        let mut reader = BufReader::new(b"e\ne\n");
        let (mut board, player_1, player_2) = setup::new_game(&mut reader);
        run_game(&mut board, player_1, player_2);

        assert!(rules::is_game_over(&mut board));
    }

    #[test]
    fn plays_two_games_and_exits() {
        let mut reader = BufReader::new(b"e\ne\ny\ne\ne\nn\n");
        start_playing_session(&mut reader);

        assert!(true);
    }
}
