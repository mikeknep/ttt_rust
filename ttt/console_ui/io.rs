use std::io;
use super::super::core::board::{Board, Token};
use super::super::core::{rules, simple_ai, unbeatable_ai};

pub fn get_next_move(board: &Board, _token: Token) -> uint {
    println!("Where do you want to play?");
    loop {
        let reader = &mut io::stdin();
        let input = get_input(reader);
        let input_num: Option<uint> = from_str(input.as_slice().trim());
        match input_num {
            Some(position) if rules::is_valid_position(position, board) => return position,
            _ => println!("That is not a valid position. Please select an open spot on the board as an integer.")
        };
    }
}


pub fn get_player_decision_maker<R: Buffer>(player_number: uint, reader: &mut R) -> fn(&Board, Token) -> uint {
    println!("What type of player is player {}? (h)uman, (e)asy computer, or (d)ifficult computer", player_number);
    loop {
        let mut input = get_input(reader);
        match input.shift_char().unwrap() {
            'h' => return get_next_move,
            'e' => return simple_ai::choose_first_available_cell,
            'd' => return unbeatable_ai::choose_best_available_cell,
             _  => println!("That is not a valid player type. Please enter \"h\" for human, \"e\" for easy computer, or \"d\" for difficult computer.")
        };
    }
}

pub fn get_play_again<R: Buffer>(reader: &mut R) -> bool {
    println!("Do you want to play again? (y)es / (n)o");
    loop {
        let mut input = get_input(reader);
        match input.shift_char().unwrap() {
            'y' => return true,
            'n' => return false,
             _  => println!("That is not a valid response. Please enter \"y\" to play again or \"n\" to exit.")
        };
    }
}

fn get_input<R: Buffer>(reader: &mut R) -> String {
    reader.read_line().ok().expect("There was a problem reading your input.")
}
