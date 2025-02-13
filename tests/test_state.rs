use wordle::state::State;
use wordle::types::{Color, ColoredLetter, ColoredLetters, Rule, Rules};

#[test]
fn test_check_guess_false() {
    let mut state = State::new(String::from("grape"));
    let guess = String::from("apple");

    let res = state.check_guess(guess);
    assert!(!res);

}

#[test]
fn test_check_guess_true() {
    let mut state = State::new(String::from("grape"));
    let guess = String::from("grape");

    let res = state.check_guess(guess);
    assert!(res);
}

#[test]
fn test_check_guess_colored_letters1() {
    let mut state = State::new(String::from("grape"));
    let guess = String::from("apple");

    let mut expected = ColoredLetters::new();
    expected.add(ColoredLetter::new('a', Color::Yellow));
    expected.add(ColoredLetter::new('p', Color::Yellow));
    expected.add(ColoredLetter::new('p', Color::Gray));
    expected.add(ColoredLetter::new('l', Color::Gray));
    expected.add(ColoredLetter::new('e', Color::Green));

    let _ = state.check_guess(guess);

    assert_eq!(state.colored_letters(), expected);
}
#[test]
fn test_check_guess_colored_letters2() {
    let mut state = State::new(String::from("apple"));
    let guess = String::from("grape");

    let mut expected = ColoredLetters::new();
    expected.add(ColoredLetter::new('g', Color::Gray));
    expected.add(ColoredLetter::new('r', Color::Gray));
    expected.add(ColoredLetter::new('a', Color::Yellow));
    expected.add(ColoredLetter::new('p', Color::Yellow));
    expected.add(ColoredLetter::new('e', Color::Green));

    let _ = state.check_guess(guess);

    assert_eq!(state.colored_letters(), expected);
}

#[test]
fn test_check_guess_colored_letters3() {
    let mut state = State::new(String::from("about"));
    let guess = String::from("again");

    let mut expected = ColoredLetters::new();
    expected.add(ColoredLetter::new('a', Color::Green));
    expected.add(ColoredLetter::new('g', Color::Gray));
    expected.add(ColoredLetter::new('a', Color::Gray));
    expected.add(ColoredLetter::new('i', Color::Gray));
    expected.add(ColoredLetter::new('n', Color::Gray));

    let _ = state.check_guess(guess);

    assert_eq!(state.colored_letters(), expected);
}

#[test]
fn test_check_guess_rules1() {
    let mut state = State::new(String::from("grape"));
    let guess = String::from("apple");

    let mut expected = Rules::new();
    expected.add(Rule::new(5, 'a', Color::Yellow));
    expected.add(Rule::new(5, 'p', Color::Yellow));
    expected.add(Rule::new(5, 'p', Color::Gray));
    expected.add(Rule::new(5,'l', Color::Gray));
    expected.add(Rule::new(4,'e', Color::Green));
    expected.add(Rule::new(5,'e', Color::Yellow));


    let _ = state.check_guess(guess);

    assert_eq!(state.rules(), expected);
}
#[test]
fn test_check_guess_rules2() {
    let mut state = State::new(String::from("apple"));
    let guess = String::from("grape");

    let mut expected = Rules::new();
    expected.add(Rule::new(5,'g', Color::Gray));
    expected.add(Rule::new(5,'r', Color::Gray));
    expected.add(Rule::new(5,'a', Color::Yellow));
    expected.add(Rule::new(5,'p', Color::Yellow));
    expected.add(Rule::new(4,'e', Color::Green));
    expected.add(Rule::new(5,'e', Color::Yellow));


    let _ = state.check_guess(guess);

    assert_eq!(state.rules(), expected);
}

#[test]
fn test_check_guess_rules3() {
    let mut state = State::new(String::from("about"));
    let guess = String::from("again");

    let mut expected = Rules::new();
    expected.add(Rule::new(0,'a', Color::Green));
    expected.add(Rule::new(5,'a', Color::Yellow));
    expected.add(Rule::new(5,'g', Color::Gray));
    expected.add(Rule::new(5,'a', Color::Gray));
    expected.add(Rule::new(5,'i', Color::Gray));
    expected.add(Rule::new(5,'n', Color::Gray));

    let _ = state.check_guess(guess);

    assert_eq!(state.rules(), expected);
}

#[test]
fn test_check_guess_colored_letters_2_consecutive() {
    let mut state = State::new(String::from("about"));
    let guess1 = String::from("grape");
    let guess2 = String::from("again");

    let mut expected = ColoredLetters::new();
    expected.add(ColoredLetter::new('a', Color::Green));
    expected.add(ColoredLetter::new('g', Color::Gray));
    expected.add(ColoredLetter::new('a', Color::Gray));
    expected.add(ColoredLetter::new('i', Color::Gray));
    expected.add(ColoredLetter::new('n', Color::Gray));

    let _ = state.check_guess(guess1);
    let _ = state.check_guess(guess2);

    assert_eq!(state.colored_letters(), expected);
}

#[test]
fn test_check_guess_rules_2_consecutive() {
    let mut state = State::new(String::from("about"));
    let guess1 = String::from("grape");
    let guess2 = String::from("again");

    let mut expected = Rules::new();
    expected.add(Rule::new(0,'a', Color::Green));
    expected.add(Rule::new(5,'a', Color::Yellow));
    expected.add(Rule::new(5,'g', Color::Gray));
    expected.add(Rule::new(5,'a', Color::Gray));
    expected.add(Rule::new(5,'i', Color::Gray));
    expected.add(Rule::new(5,'n', Color::Gray));
    expected.add(Rule::new(5,'r', Color::Gray));
    expected.add(Rule::new(5,'p', Color::Gray));
    expected.add(Rule::new(5,'e', Color::Gray));

    let _ = state.check_guess(guess1);
    let _ = state.check_guess(guess2);

    assert_eq!(state.rules(), expected);
}

#[test]
fn test_check_guess_rules_2_consecutive2() {
    let mut state = State::new(String::from("apple"));
    let guess1 = String::from("mmmpp");
    let guess2 = String::from("pxpxx");

    let mut expected = Rules::new();
    expected.add(Rule::new(2,'p', Color::Green));
    expected.add(Rule::new(5,'p', Color::Yellow));
    expected.add(Rule::new(5,'p', Color::Yellow));
    expected.add(Rule::new(5,'x', Color::Gray));
    expected.add(Rule::new(5,'m', Color::Gray));

    let _ = state.check_guess(guess1);
    let _ = state.check_guess(guess2);

    assert_eq!(state.rules(), expected);
}

#[test]
fn test_lie_with_gray_found_in_rules() {
    let mut state = State::new(String::from("about"));
    let guess = String::from("heart");
    state.check_guess(guess);
    let rules = state.rules().clone();
    let guess2 = String::from("guess");
    state.check_guess(guess2);
    let mut indexes = vec![2];
    let result = state.lie_with_gray_letter(&mut indexes, 2, &rules);
    let expected = (false, state.colored_letters());
    assert_eq!(result, expected);
}

#[test]
fn test_lie_with_gray_to_green() {
    let mut state = State::new(String::from("about"));
    let guess = String::from("heart");
    state.check_guess(guess);
    let rules = state.rules().clone();
    let guess2 = String::from("grass");
    state.check_guess(guess2);
    let mut indexes = vec![2];
    let result = state.lie_with_gray_letter(&mut indexes, 2, &rules);
    let mut false_letters = ColoredLetters::new();
    false_letters.add(ColoredLetter::new('g', Color::Gray));
    false_letters.add(ColoredLetter::new('r', Color::Gray));
    false_letters.add(ColoredLetter::new('a', Color::Green));
    false_letters.add(ColoredLetter::new('s', Color::Gray));
    false_letters.add(ColoredLetter::new('s', Color::Gray));
    let expected = (true, false_letters);
    assert_eq!(result, expected);
}
#[test]
fn test_lie_with_gray_to_yellow() {
    let mut state = State::new(String::from("about"));
    let guess = String::from("heart");
    state.check_guess(guess);
    let rules = state.rules().clone();
    let guess2 = String::from("guess");
    state.check_guess(guess2);
    let mut indexes = vec![3];
    let result = state.lie_with_gray_letter(&mut indexes, 3, &rules);
    let mut false_letters = ColoredLetters::new();
    false_letters.add(ColoredLetter::new('g', Color::Gray));
    false_letters.add(ColoredLetter::new('u', Color::Yellow));
    false_letters.add(ColoredLetter::new('e', Color::Gray));
    false_letters.add(ColoredLetter::new('s', Color::Yellow));
    false_letters.add(ColoredLetter::new('s', Color::Gray));
    let expected = (true, false_letters);
    assert_eq!(result, expected);
}
#[test]
fn test_lie_with_gray_all_letters_known() {
    let mut state = State::new(String::from("about"));
    let guess = String::from("tuoba");
    state.check_guess(guess);
    let rules = state.rules().clone();
    let guess2 = String::from("guess");
    state.check_guess(guess2);
    let mut indexes = vec![2];
    let result = state.lie_with_gray_letter(&mut indexes, 2, &rules);
    let expected = (false, state.colored_letters());
    assert_eq!(result, expected);
}

#[test]
fn test_lie_with_yellow_to_gray() {
    let mut state = State::new(String::from("about"));
    let guess = String::from("honey");
    state.check_guess(guess);
    let rules = state.rules().clone();
    let guess2 = String::from("guess");
    state.check_guess(guess2);
    let mut indexes = vec![1];
    let result = state.lie_with_yellow_letter(&mut indexes, 1, &rules);
    let mut false_letters = ColoredLetters::new();
    false_letters.add(ColoredLetter::new('g', Color::Gray));
    false_letters.add(ColoredLetter::new('u', Color::Gray));
    false_letters.add(ColoredLetter::new('e', Color::Gray));
    false_letters.add(ColoredLetter::new('s', Color::Gray));
    false_letters.add(ColoredLetter::new('s', Color::Gray));
    let expected = (true, false_letters);
    assert_eq!(result, expected);
}

#[test]
fn test_lie_with_yellow_green_in_same_pos() {
    let mut state = State::new(String::from("about"));
    let guess = String::from("xbuxx");
    state.check_guess(guess);
    let rules = state.rules().clone();
    let guess2 = String::from("guess");
    state.check_guess(guess2);
    let mut indexes = vec![1];
    let result = state.lie_with_yellow_letter(&mut indexes, 1, &rules);
    let expected = (false, state.colored_letters());
    assert_eq!(result, expected);
}
#[test]
fn test_lie_with_yellow_not_enough_grays_and_yellows_left() {
    let mut state = State::new(String::from("about"));
    let guess = String::from("xxuxx");
    state.check_guess(guess);
    let rules = state.rules().clone();
    let guess2 = String::from("auobt");
    state.check_guess(guess2);
    let mut indexes = vec![1];
    let result = state.lie_with_yellow_letter(&mut indexes, 1, &rules);
    let expected = (false, state.colored_letters());
    assert_eq!(result, expected);
}

#[test]
fn test_lie_with_yellow_to_green() {
    let mut state = State::new(String::from("about"));
    let guess = String::from("xxuxx");
    state.check_guess(guess);
    let rules = state.rules().clone();
    let guess2 = String::from("guess");
    state.check_guess(guess2);
    let mut indexes = vec![1];
    let result = state.lie_with_yellow_letter(&mut indexes, 1, &rules);
    let mut false_letters = ColoredLetters::new();
    false_letters.add(ColoredLetter::new('g', Color::Gray));
    false_letters.add(ColoredLetter::new('u', Color::Green));
    false_letters.add(ColoredLetter::new('e', Color::Gray));
    false_letters.add(ColoredLetter::new('s', Color::Gray));
    false_letters.add(ColoredLetter::new('s', Color::Gray));
    let expected = (true, false_letters);
    assert_eq!(result, expected);
}

#[test]
fn test_lie_with_green_in_rules_as_green() {
    let mut state = State::new(String::from("about"));
    let guess = String::from("abort");
    state.check_guess(guess);
    let rules = state.rules().clone();
    let guess2 = String::from("alive");
    state.check_guess(guess2);
    let mut indexes = vec![0];
    let result = state.lie_with_green_letter(&mut indexes, 0, &rules);
    let expected = (false, state.colored_letters());
    assert_eq!(result, expected);
}

#[test]
fn test_lie_with_green_in_rules_as_yellow() {
    let mut state = State::new(String::from("about"));
    let guess = String::from("movie");
    state.check_guess(guess);
    let rules = state.rules().clone();
    let guess2 = String::from("phone");
    state.check_guess(guess2);
    let mut indexes = vec![2];
    let result = state.lie_with_green_letter(&mut indexes, 2, &rules);
    let expected = (false, state.colored_letters());
    assert_eq!(result, expected);
}

#[test]
fn test_lie_with_green_to_gray() {
    let mut state = State::new(String::from("about"));
    let guess = String::from("xxxxx");
    state.check_guess(guess);
    let rules = state.rules().clone();
    let guess2 = String::from("abort");
    state.check_guess(guess2);
    let mut indexes = vec![1];
    let result = state.lie_with_green_letter(&mut indexes, 1, &rules);
    let mut false_letters = ColoredLetters::new();
    false_letters.add(ColoredLetter::new('a', Color::Green));
    false_letters.add(ColoredLetter::new('b', Color::Gray));
    false_letters.add(ColoredLetter::new('o', Color::Green));
    false_letters.add(ColoredLetter::new('r', Color::Gray));
    false_letters.add(ColoredLetter::new('t', Color::Green));
    let expected = (true, false_letters);
    assert_eq!(result, expected);
}