use std::io;
use super::super::core::board::{Board, Token};
use super::super::core::rules::is_valid_position;
use super::super::core::simple_ai::choose_first_available_cell;
use super::super::core::unbeatable_ai::choose_best_available_cell;

pub fn get_next_move(board: &Board, _token: Token) -> uint {
    println!("Where do you want to play?");
    let mut reader = io::stdin();
    loop {
        let input = reader.read_line().ok().expect("There was a problem reading your input.");
        let input_num: Option<uint> = from_str(input.as_slice().trim());
        match input_num {
            Some(position) if is_valid_position(position, board) => return position,
            _ => println!("That is not a valid position. Please select an open spot on the board as an integer.")
        };
    }
}


pub fn get_player_decision_maker(player_number: uint) -> fn(&Board, Token) -> uint {
    println!("What type of player is player {}? (h)uman, (e)asy computer, or (d)ifficult computer", player_number);
    let mut reader = io::stdin();
    loop {
        let input = reader.read_line().ok().expect("There was a problem reading your input.");
        let input_char: &str = input.as_slice().trim();
        match input_char {
            "h" => return get_next_move,
            "e" => return choose_first_available_cell,
            "d" => return choose_best_available_cell,
            _ => println!("That is not a valid player type. Please enter \"h\" for human, \"e\" for easy computer, or \"d\" for difficult computer.")
        };
    }
}
