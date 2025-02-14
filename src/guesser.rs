use crate::state::State;
use crate::types::{Color, ColoredLetters, Rule, Rules};

#[derive(Debug, Clone)]
pub struct Guesser {
    words: Vec<String>,
    rules: Rules,
}
impl Guesser {
    pub fn new(words: Vec<String>) -> Guesser {
        let rules = Rules::new();
        Self { words, rules }
    }

    pub fn add_to_rules(&mut self, colored_letters: ColoredLetters) {
        let word_length = colored_letters.size();
        let mut ind = 0;
        for colored_letter in colored_letters.clone().into_iter() {
            match  colored_letter.color() {
                Color::Yellow => {
                    if self.rules.colored_letter_occurrences(&colored_letter) <
                        colored_letters.letter_occurrences_with_color(colored_letter.letter(), Color::Yellow) {
                        self.rules.add(Rule::new_with_colored_letter(word_length, colored_letter.clone()));
                    }
                },
                Color::Green => {
                    if !self.rules.contains(ind, colored_letter.clone()) {
                        self.rules.add(Rule::new_with_colored_letter(ind, colored_letter.clone()))
                    }
                    if self.rules.letter_occurrences_with_color(colored_letter.letter(), Color::Yellow) == 0 {
                        self.rules.add(Rule::new(word_length, colored_letter.letter(), Color::Yellow));
                    }
                },
                _ => self.rules.add(Rule::new_with_colored_letter(word_length, colored_letter)),
            }
            ind += 1;
        }
    }

    pub fn rules(&self) -> Rules {
        self.rules.clone()
    }
    pub fn is_contradiction(rules: &Rules,  colored_letters: &ColoredLetters) -> bool {
        !rules.all_green_used(colored_letters)
        || !rules.all_yellow_used(colored_letters)
        || rules.gray_used(colored_letters)
    }

    fn is_contradiction_word(rules: &Rules, guess: String) -> bool {
        !rules.all_green_used_in_word(&guess)
            || !rules.all_yellow_used_in_word(&guess)
            || rules.gray_used_in_word(&guess)
    }

    fn count_eliminated_by_guess(&self, guess: String, secret_word: String, rules: Rules) -> usize {
        let mut eliminated:usize = 0;
        for word in self.words.iter() {
            if word.eq(&guess) || word.eq(&secret_word) {
                continue;
            }
            let mut state = State::new_with_rules(secret_word.clone(), rules.clone());
            state.check_guess(word.clone());
            if Guesser::is_contradiction(&rules, &state.colored_letters()) {
                eliminated += 1;
            }
        }
        eliminated
    }

    fn secret_word_loop(&self, guess: &String) -> usize {
        let mut eliminated_words = 0;
        for secret_word in self.words.iter() {
            if secret_word == guess {
                continue;
            }
            let mut state = State::new(secret_word.clone());
            state.check_guess(guess.clone());
            eliminated_words += self.count_eliminated_by_guess(guess.clone(), secret_word.clone(), state.rules());
        }
        eliminated_words
    }

    pub fn guess_with_most_eliminated(&mut self) -> String {
        let mut best_guess = String::new();
        let mut best_guess_eliminated_count:usize = 0;
        for guess in self.words.iter() {
            if Guesser::is_contradiction_word(&self.rules, guess.clone()) {
                continue;
            }
            let current_guess_eliminated_count = self.secret_word_loop(guess);
            if current_guess_eliminated_count > best_guess_eliminated_count {
                best_guess = guess.clone();
                best_guess_eliminated_count = current_guess_eliminated_count;
            }
        }

        best_guess
    }

    pub fn guess(&mut self) -> String {
        let mut new_words:Vec<String> = vec![];
        for guess in self.words.iter().cloned() {
            if Guesser::is_contradiction_word(&self.rules, guess.clone()) {
                continue;
            }
            new_words.push(guess);
        }
        if new_words.is_empty() {
            return String::new();
        }
        self.words = new_words;
        self.words[0].clone()
    }
}