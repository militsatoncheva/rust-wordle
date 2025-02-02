use std::io;
use rand::Rng;
use crate::file_utils;
use crate::print_utils::print_guess;
use crate::state::State;

fn read_guess(guess: &mut String) {
    println!("Please enter your guess:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    guess.clear();
    guess.push_str(input.trim())
}

fn check_guess(guess: &mut String, wordlist: &Vec<String>){
    while guess.len() != 5{
        println!("Your guess must be a 5 letter word!");
        read_guess(guess);
    }
    while !wordlist.contains(&guess){
        println!("Your guess is not in the wordlist!");
        read_guess(guess);
    }
}

pub fn normal_game() {
    let words = file_utils::extract_words(String::from("src/words.txt"));
    let secret_word = file_utils::choose_word(&words);
    let mut state = State::new(secret_word.to_string());
    loop {
        let mut guess = String::new();
        read_guess(&mut guess);
        check_guess(&mut guess, &words);
        let result = state.check_guess(guess);
        print_guess(state.colored_letters());
        if  result {
            println!("You win!");
            break;
        }
    }
}

pub fn hard_mode() {
    let words = file_utils::extract_words(String::from("src/words.txt"));
    let secret_word = file_utils::choose_word(&words);
    let mut state = State::new(secret_word.to_string());
    let mut lied = false;
    loop {
        let mut guess = String::new();
        read_guess(&mut guess);
        check_guess(&mut guess, &words);
        let result = state.check_guess(guess);
        if lied || state.rules().empty() {
            print_guess(state.colored_letters());
            if  result {
                println!("You win!");
                break;
            }
        }
        else {
            let mut rng= rand::rng();
            let number = rng.random_range(1..=3);
            if number == 1 {
                let mut valid_indexes = (0..secret_word.len()).collect::<Vec<usize>>();
                let (success, false_letters) = state.lie(&mut valid_indexes);
                if success {
                    lied = true;
                    print_guess(false_letters);
                    continue;
                }
            }
            print_guess(state.colored_letters());
        }
    }
}