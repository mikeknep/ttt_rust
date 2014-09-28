use std::io;
use super::super::core::board::{Board, Token};
use super::super::core::{rules, simple_ai, unbeatable_ai};

pub fn get_next_move(board: &Board, _token: Token) -> uint {
    println!("Where do you want to play?");
    let mut reader = io::stdin();
    loop {
        let input = reader.read_line().ok().expect("There was a problem reading your input.");
        let input_num: Option<uint> = from_str(input.as_slice().trim());
        match input_num {
            Some(position) if rules::is_valid_position(position, board) => return position,
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
            "e" => return simple_ai::choose_first_available_cell,
            "d" => return unbeatable_ai::choose_best_available_cell,
            _ => println!("That is not a valid player type. Please enter \"h\" for human, \"e\" for easy computer, or \"d\" for difficult computer.")
        };
    }
}

pub fn get_play_again() -> bool {
    println!("Do you want to play again? (y)es / (n)o");
    let mut reader = io::stdin();
    loop {
        let input = reader.read_line().ok().expect("There was a problem reading your input.");
        let input_char: &str = input.as_slice().trim();
        match input_char {
            "y" => return true,
            "n" => return false,
            _ => println!("That is not a valid response. Please enter \"y\" to play again or \"n\" to exit.")
        };
    }
}
