use wordle::guesser::Guesser;
use wordle::types::{Color, ColoredLetter, ColoredLetters, Rule, Rules};

#[test]
pub fn test_is_contradiction_is_not() {
    let mut rules = Rules::new();
    rules.add(Rule::new(5, 'a', Color::Yellow));
    rules.add(Rule::new(5, 'p', Color::Yellow));
    rules.add(Rule::new(5, 'p', Color::Gray));
    rules.add(Rule::new(5,'l', Color::Gray));
    rules.add(Rule::new(4,'e', Color::Green));
    rules.add(Rule::new(5,'e', Color::Yellow));

    let mut colored_letters = ColoredLetters::new();
    colored_letters.add(ColoredLetter::new('p', Color::Yellow));
    colored_letters.add(ColoredLetter::new('a', Color::Yellow));
    colored_letters.add(ColoredLetter::new('s', Color::Gray));
    colored_letters.add(ColoredLetter::new('t', Color::Gray));
    colored_letters.add(ColoredLetter::new('e', Color::Green));
    let result = Guesser::is_contradiction(&rules, &colored_letters);

    assert_eq!(result, false);
}

#[test]
pub fn test_is_contradiction_missing_green() {
    let mut rules = Rules::new();
    rules.add(Rule::new(5, 'a', Color::Yellow));
    rules.add(Rule::new(5, 'p', Color::Yellow));
    rules.add(Rule::new(5, 'p', Color::Gray));
    rules.add(Rule::new(5,'l', Color::Gray));
    rules.add(Rule::new(4,'e', Color::Green));
    rules.add(Rule::new(5,'e', Color::Yellow));

    let mut colored_letters = ColoredLetters::new();
    colored_letters.add(ColoredLetter::new('p', Color::Yellow));
    colored_letters.add(ColoredLetter::new('a', Color::Yellow));
    colored_letters.add(ColoredLetter::new('s', Color::Gray));
    colored_letters.add(ColoredLetter::new('t', Color::Gray));
    colored_letters.add(ColoredLetter::new('a', Color::Gray));
    let result = Guesser::is_contradiction(&rules, &colored_letters);

    assert_eq!(result, true);
}

#[test]
pub fn test_is_contradiction_missing_yellow() {
    let mut rules = Rules::new();
    rules.add(Rule::new(5, 'a', Color::Yellow));
    rules.add(Rule::new(5, 'p', Color::Yellow));
    rules.add(Rule::new(5, 'p', Color::Gray));
    rules.add(Rule::new(5,'l', Color::Gray));
    rules.add(Rule::new(4,'e', Color::Green));
    rules.add(Rule::new(5,'e', Color::Yellow));

    let mut colored_letters = ColoredLetters::new();
    colored_letters.add(ColoredLetter::new('p', Color::Yellow));
    colored_letters.add(ColoredLetter::new('o', Color::Gray));
    colored_letters.add(ColoredLetter::new('s', Color::Gray));
    colored_letters.add(ColoredLetter::new('t', Color::Gray));
    colored_letters.add(ColoredLetter::new('e', Color::Green));
    let result = Guesser::is_contradiction(&rules, &colored_letters);

    assert_eq!(result, true);
}

#[test]
pub fn test_is_contradiction_used_gray() {
    let mut rules = Rules::new();
    rules.add(Rule::new(5, 'a', Color::Yellow));
    rules.add(Rule::new(5, 'p', Color::Yellow));
    rules.add(Rule::new(5, 'p', Color::Gray));
    rules.add(Rule::new(5,'l', Color::Gray));
    rules.add(Rule::new(4,'e', Color::Green));
    rules.add(Rule::new(5,'e', Color::Yellow));

    let mut colored_letters = ColoredLetters::new();
    colored_letters.add(ColoredLetter::new('p', Color::Yellow));
    colored_letters.add(ColoredLetter::new('l', Color::Gray));
    colored_letters.add(ColoredLetter::new('a', Color::Green));
    colored_letters.add(ColoredLetter::new('t', Color::Gray));
    colored_letters.add(ColoredLetter::new('e', Color::Green));
    let result = Guesser::is_contradiction(&rules, &colored_letters);

    assert_eq!(result, true);
}

#[test]
fn test_add_to_rules_all_new() {
    let words = vec![String::from("apple"), String::from("beach"), String::from("clock"), String::from("dream"),
                     String::from("eagle"), String::from("flame"), String::from("ideal"),
                     String::from("ghost"), String::from("honey"), String::from("happy")];
    let mut guesser = Guesser::new(words);

    let mut colored_letters = ColoredLetters::new();
    colored_letters.add(ColoredLetter::new('a', Color::Yellow));
    colored_letters.add(ColoredLetter::new('p', Color::Yellow));
    colored_letters.add(ColoredLetter::new('p', Color::Gray));
    colored_letters.add(ColoredLetter::new('l', Color::Gray));
    colored_letters.add(ColoredLetter::new('e', Color::Green));

    guesser.add_to_rules(colored_letters);

    let mut expected_rules = Rules::new();
    expected_rules.add(Rule::new(5, 'a', Color::Yellow));
    expected_rules.add(Rule::new(5, 'p', Color::Yellow));
    expected_rules.add(Rule::new(5, 'p', Color::Gray));
    expected_rules.add(Rule::new(5,'l', Color::Gray));
    expected_rules.add(Rule::new(4,'e', Color::Green));
    expected_rules.add(Rule::new(5,'e', Color::Yellow));

    assert_eq!(guesser.rules(), expected_rules);
}

#[test]
fn test_add_to_rules_some_new() {
    let words = vec![];
    let mut guesser = Guesser::new(words);

    let mut colored_letters = ColoredLetters::new();
    colored_letters.add(ColoredLetter::new('a', Color::Yellow));
    colored_letters.add(ColoredLetter::new('p', Color::Yellow));
    colored_letters.add(ColoredLetter::new('p', Color::Gray));
    colored_letters.add(ColoredLetter::new('l', Color::Gray));
    colored_letters.add(ColoredLetter::new('e', Color::Green));

    guesser.add_to_rules(colored_letters);

    let mut colored_letters2 = ColoredLetters::new();
    colored_letters2.add(ColoredLetter::new('e', Color::Gray));
    colored_letters2.add(ColoredLetter::new('a', Color::Yellow));
    colored_letters2.add(ColoredLetter::new('g', Color::Yellow));
    colored_letters2.add(ColoredLetter::new('l', Color::Gray));
    colored_letters2.add(ColoredLetter::new('e', Color::Green));

    guesser.add_to_rules(colored_letters2);

    let mut expected_rules = Rules::new();
    expected_rules.add(Rule::new(5, 'a', Color::Yellow));
    expected_rules.add(Rule::new(5, 'p', Color::Yellow));
    expected_rules.add(Rule::new(5, 'p', Color::Gray));
    expected_rules.add(Rule::new(5,'l', Color::Gray));
    expected_rules.add(Rule::new(4,'e', Color::Green));
    expected_rules.add(Rule::new(5,'e', Color::Yellow));
    expected_rules.add(Rule::new(5,'e', Color::Gray));
    expected_rules.add(Rule::new(5,'g', Color::Yellow));


    assert_eq!(guesser.rules(), expected_rules);

}