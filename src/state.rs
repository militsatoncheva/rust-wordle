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

    pub fn colored_letters(&self) -> ColoredLetters {
        self.colored_letters.clone()
    }
    pub fn rules(&self) -> Rules {
        self.rules.clone()
    }
}