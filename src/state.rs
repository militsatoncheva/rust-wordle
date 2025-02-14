use rand::Rng;
use crate::types::{Color, ColoredLetter, ColoredLetters, Rule, Rules};

pub struct State {
    secret_word: String,
    colored_letters: ColoredLetters,
    rules: Rules,
}

impl State {
    pub fn new(secret_word: String) -> State {
        let colored_letters = ColoredLetters::new();
        let rules = Rules::new();
        Self {secret_word, colored_letters, rules}
    }
    pub fn new_with_rules(secret_word: String, rules: Rules) -> State {
        let colored_letters = ColoredLetters::new();
        Self {secret_word, colored_letters, rules}
    }
    pub fn secret_word(&self) -> &String{
        &self.secret_word
    }
    pub fn colored_letters(&self) -> ColoredLetters {
        self.colored_letters.clone()
    }
    pub fn rules(&self) -> Rules {
        self.rules.clone()
    }
}

impl State {
    fn check_greens(&mut self, guess: String) {
        for i in 0..guess.len() {
            let guess_letter = guess.chars().nth(i).unwrap();
            let secret_word_letter = self.secret_word.chars().nth(i).unwrap();
            let in_guess = self.secret_word.chars()
                .filter(|&c| c == guess_letter)
                .count();
            if guess_letter == secret_word_letter {
                self.colored_letters.add(ColoredLetter::new(guess_letter, Color::Green));
                self.rules.add(Rule::new(i, guess_letter, Color::Green));
                if self.rules.letter_occurrences_with_color(guess_letter, Color::Yellow) < in_guess  {
                    self.rules.add(Rule::new(self.secret_word.len(), guess_letter, Color::Yellow));
                }
                continue;
            }
            self.colored_letters.add(ColoredLetter::new(guess_letter, Color::White));
        }
    }

    fn more_occurrences_in_guess_than_colored_letters(&self, letter: char) -> bool {
        let in_secret_word = self.secret_word.chars()
            .filter(|&c| c == letter)
            .count();
        let in_colored_letters = self.colored_letters.letter_occurrences_with_color(letter, Color::Green)
            + self.colored_letters.letter_occurrences_with_color(letter, Color::Yellow);
        in_secret_word > in_colored_letters
    }
    fn check_yellows(&mut self, guess: String) {
        let mut i =0;
        for colored_letter in self.colored_letters.letters() {
            if colored_letter.color() == Color::Green {
                i+=1;
                continue;
            }
            else if guess.contains(colored_letter.letter())
                && self.more_occurrences_in_guess_than_colored_letters(colored_letter.letter()) {
                let in_guess = self.secret_word.chars()
                    .filter(|&c| c == colored_letter.letter())
                    .count();
                self.colored_letters.replace_color(i, Color::Yellow);
                if self.rules.letter_occurrences_with_color(colored_letter.letter(), Color::Yellow) < in_guess  {
                    self.rules.add(Rule::new(self.secret_word.len(), colored_letter.letter(), Color::Yellow));
                }
            }
            else {
                self.colored_letters.replace_color(i, Color::Gray);
                self.rules.add(Rule::new(self.secret_word.len(), colored_letter.letter(), Color::Gray));
            }
            i += 1;
        }
    }
    pub fn check_guess(&mut self, guess: String) -> bool {
        self.colored_letters = ColoredLetters::new();
        self.check_greens(guess.clone());
        self.check_yellows(guess.clone());
        guess == self.secret_word
    }
}

impl State {
    pub fn lie_with_green_letter(&mut self, valid_indexes: &mut Vec<usize>, element_ind: usize, rules: &Rules) -> (bool, ColoredLetters){
        let colored_letter: ColoredLetter = self.colored_letters[element_ind].clone();
        if rules.contains(element_ind, colored_letter.clone())
        || rules.letter_occurrences_with_color(colored_letter.letter(), Color::Yellow) > 0  {
            let mut new_indexes = valid_indexes.iter().filter(|c| **c != element_ind).copied().collect();
            return self.lie(&mut new_indexes, rules);
        }
        let mut false_letters = self.colored_letters.clone();
        false_letters.replace_color(element_ind, Color::Gray);
        (true, false_letters)
    }

    pub fn lie_with_yellow_letter(&mut self, valid_indexes: &mut Vec<usize>, element_ind: usize, rules: &Rules) -> (bool, ColoredLetters){
        let colored_letter: ColoredLetter = self.colored_letters[element_ind].clone();
        let mut false_letters = self.colored_letters.clone();
        if rules.letter_occurrences(colored_letter.letter()) == 0 {
            false_letters.replace_color(element_ind, Color::Gray);
            return (true, false_letters)
        }
        else if rules
            .clone()
            .into_iter()
            .filter(|c| c.position() == element_ind && c.color() == Color::Green).count() > 0
            || (self.colored_letters.letters_with_color_count(Color::Gray) == 0
                && self.colored_letters.letters_with_color_count(Color::Yellow) < 3){
            let mut new_indexes = valid_indexes.iter().filter(|c| **c != element_ind).copied().collect();
            return self.lie(&mut new_indexes, rules);
        }
        false_letters.replace_color(element_ind, Color::Green);
        (true, false_letters)
    }

    pub fn lie_with_gray_letter(&mut self, valid_indexes: &mut Vec<usize>, element_ind: usize, rules: &Rules) -> (bool, ColoredLetters){
        let colored_letter: ColoredLetter = self.colored_letters[element_ind].clone();
        let mut false_letters = self.colored_letters.clone();

        if rules.color_occurrences(Color::Yellow) == self.secret_word.len()
            || (self.colored_letters.letters_with_color_count(Color::Gray) == 1
                && self.colored_letters.letters_with_color_count(Color::Yellow) < 2){
            let mut new_indexes = valid_indexes.iter().filter(|c| **c != element_ind).copied().collect();
            return self.lie(&mut new_indexes, rules);
        }
        else if rules.letter_occurrences(colored_letter.letter()) == 0 {
            false_letters.replace_color(element_ind, Color::Yellow);
            return (true, false_letters)
        }
        else if rules.letter_occurrences_with_color(colored_letter.letter(), Color::Gray) > 0
        || rules
            .clone()
            .into_iter().
            filter(|c| c.position() == element_ind && c.color() == Color::Green).count() > 0{
            let mut new_indexes = valid_indexes.iter().filter(|c| **c != element_ind).copied().collect();
            return self.lie(&mut new_indexes, rules);
        }

        false_letters.replace_color(element_ind, Color::Green);
        (true, false_letters)
    }
    pub fn lie(&mut self, valid_indexes: &mut Vec<usize>, rules_before_guess: &Rules) -> (bool, ColoredLetters) {
        if valid_indexes.is_empty() {
            return (false, self.colored_letters.clone());
        }
        let mut rng= rand::rng();
        let ind = rng.random_range(0..valid_indexes.len());
        match self.colored_letters.letters()[ind].color() {
            Color::Green => self.lie_with_green_letter(valid_indexes, ind, &rules_before_guess),
            Color::Yellow => self.lie_with_yellow_letter(valid_indexes, ind, &rules_before_guess),
            Color::Gray => self.lie_with_gray_letter(valid_indexes, ind, &rules_before_guess),
            _ => panic!("unknown color"),
        }
    }
}