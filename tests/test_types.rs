use wordle::types::{Color, ColoredLetter, ColoredLetters, Rule, Rules};

#[test]
fn test_colored_letters() {
    let colored_letter1 = ColoredLetter::new('a', Color::Green);
    let colored_letter2 = ColoredLetter::new('b', Color::Yellow);
    let colored_letter3 = ColoredLetter::new('c', Color::Gray);

    let mut colored_letters = ColoredLetters::new();
    colored_letters.add(colored_letter1.clone());
    colored_letters.add(colored_letter2.clone());
    colored_letters.add(colored_letter3.clone());

    assert_eq!(colored_letters.letters(), vec![colored_letter1, colored_letter2, colored_letter3]);
}

#[test]
fn test_letter_occurrences_with_color() {
    let colored_letter1 = ColoredLetter::new('a', Color::Green);
    let colored_letter2 = ColoredLetter::new('b', Color::Yellow);
    let colored_letter3 = ColoredLetter::new('c', Color::Gray);

    let mut colored_letters = ColoredLetters::new();
    colored_letters.add(colored_letter1.clone());
    colored_letters.add(colored_letter2.clone());
    colored_letters.add(colored_letter3.clone());
    let letter = 'a';

    let result_green = colored_letters.letter_occurrences_with_color(letter, Color::Green);
    let result_yellow = colored_letters.letter_occurrences_with_color(letter, Color::Yellow);
    assert_eq!(result_green, 1);
    assert_eq!(result_yellow, 0);
}

#[test]
fn test_rules() {
    let rule1 = Rule::new(0, 'a', Color::Green);
    let rule2 = Rule::new(5, 'a', Color::Yellow);
    let rule3 = Rule::new(5, 'b', Color::Yellow);
    let rule4 = Rule::new(5, 'c', Color::Gray);

    let mut rules = Rules::new();
    rules.add(rule1.clone());
    rules.add(rule2.clone());
    rules.add(rule3.clone());
    rules.add(rule4.clone());

    assert_eq!(rules.rules(), vec![rule1, rule2, rule3, rule4]);
}