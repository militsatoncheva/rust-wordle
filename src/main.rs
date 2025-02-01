use std::io;
use std::io::Write;
use wordle::state::State;
use wordle::print_utils::print_guess;
use wordle::file_utils;

fn set_up() -> String {
    println!("Wordle\n");
    println!("Choose game mode:\n");
    println!("Normal game mode(n)");
    println!("Easy game mode(e)");
    println!("Hard game mode(h)");
    print!("\nPlease enter game mode: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut mode= String::new();
    io::stdin().read_line(&mut mode).expect("Failed to read line");
    String::from(mode.trim())
}

fn normal_game() {
    let words = file_utils::extract_words(String::from("src/words.txt"));
    let secret_word = file_utils::choose_word(words);
    let mut state = State::new(secret_word.to_string());
    loop {
        println!("Please enter your guess:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut guess = String::from(input.trim());

        if guess.len() != 5 {
            println!("Your guess must be a 5 letter word!");
            println!("Please enter your guess:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            guess = String::from(input.trim());
        }
        let result = state.check_guess(guess);
        print_guess(state.colored_letters());
        if  result {
            println!("You win!");
            break;
        }
    }
}
fn start_game(mode: &str) {
    match mode {
        "n" => normal_game(),
        "e" => println!("Easy mode"),
        "h" => println!("Hard mode"),
        _ => println!("Invalid mode"),
    }
}
fn main() {
    let mode= set_up();
    start_game(&mode);
}
