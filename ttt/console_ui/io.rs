use std::io;
use super::super::core::board::{Board};
use super::super::core::rules::{is_valid_position};

pub fn get_next_move(board: &Board) -> uint {
    println!("Where do you want to play?");
    let mut reader = io::stdin();
    loop {
        let input = reader.read_line().ok().expect("There was a problem reading your input.");
        let input_num: Option<int> = from_str(input.as_slice().trim());
        match input_num {
            Some(position) => if is_valid_position(position, board.cells) { return position as uint } else { println!("That is not a valid position. Please select an open spot on the board.")},
            None => println!("That is not a number. Please input an integer.")
        }
    }
}
