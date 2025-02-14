use std::io;
use rand::Rng;
use crate::file_utils;
use crate::print_utils::print_guess;
use crate::state::State;
use crate::guesser::Guesser;
use crate::types::ColoredLetters;

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
        let rules_before_current_guess = state.rules().clone();
        let result = state.check_guess(guess);
        if  result {
            print_guess(state.colored_letters());
            println!("You win!");
            break;
        }
        if lied || rules_before_current_guess.empty() {
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
                let (success, false_letters) = state.lie(&mut valid_indexes, &rules_before_current_guess);
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

fn read_colors(colors: &mut String) {
    println!("Please enter the corresponding colors:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    colors.clear();
    colors.push_str(input.trim())
}

fn check_colors(colors: &mut String, guess_length: usize) {
    while colors.len() != guess_length {
        println!("Not enough colors!");
        read_colors(colors);
    }
    while !colors.chars().all(|c| c == 'g' || c == 'y' || c == 'r') {
        println!("Valid colors: 'g' for green, 'y' for yellow or 'r' for gray");
        read_colors(colors);
    }
}

pub fn guess_mode() {
    println!("You are in guess mode!\nAfter the given guess enter 'g' for green, 'y' for yellow or 'r' for gray for each letter");
    //let words = vec![String::from("apple"), String::from("peach"), String::from("berry"), String::from("grape")]; // first grape
    //let words = vec![String::from("apple"), String::from("beach"), String::from("clock"), String::from("dream"),
    //                String::from("eagle"), String::from("flame"), String::from("ideal"),
    //                 String::from("ghost"), String::from("honey"), String::from("happy")]; // first flame secret eagle
    let words = file_utils::extract_words(String::from("src/words.txt"));
    let mut guesser = Guesser::new(words);

    loop {
        //let guess = guesser.guess_with_most_eliminated();
        let guess = guesser.guess();
        if guess.is_empty() {
            println!("No such word exist in wordlist!");
            break;
        }
        println!("{}", guess);
        let mut colors = String::new();
        read_colors(&mut colors);
        check_colors(&mut colors, guess.len());
        if colors.chars().all(|c| c == 'g') {
            println!("Thanks for playing!");
            break;
        }
        let colored_letters = ColoredLetters::from_str(guess, colors.clone());
        if Guesser::is_contradiction(&guesser.rules(), &colored_letters) {
            println!("Contradicting clues!");
            break;
        }
        guesser.add_to_rules(colored_letters);
    }
}