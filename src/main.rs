use std::io;
use std::io::Write;

use wordle::game::{normal_game, hard_mode, guess_mode};

fn start_game(mode: &str) {
    match mode {
        "n" => normal_game(),
        "h" => hard_mode(),
        "g" => guess_mode(),
        _ => println!("Invalid mode"),
    }
}
fn set_up() -> String {
    println!("Wordle\n");
    println!("Choose game mode:\n");
    println!("Normal game mode(n)");
    println!("Hard game mode(h)");
    println!("Guess mode(g)");
    print!("\nPlease enter game mode: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut mode= String::new();
    io::stdin().read_line(&mut mode).expect("Failed to read line");
    String::from(mode.trim())
}
fn main() {
    let mode= set_up();
    start_game(&mode);
}
