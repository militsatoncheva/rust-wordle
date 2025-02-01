use crate::types::{Color, ColoredLetters};
use colored::*;

pub fn print_guess(colored_letters: ColoredLetters) {
    for pair in colored_letters.letters() {
        match pair.color() {
            Color::Green => { print!("{}", pair.letter().to_string().green()); },
            Color::Yellow => { print!("{}", pair.letter().to_string().yellow()); },
            Color::Gray => { print!("{}", pair.letter().to_string().truecolor(240, 240, 240)); },
            Color::White => { print!("{}", pair.letter().to_string().on_white()); },
        }
    }
    println!();
}

