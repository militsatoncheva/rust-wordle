use std::cmp::Ordering;
use std::ops::Index;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Color {
    Green,
    Yellow,
    Gray,
    White,
}

impl Color {
    pub fn from_char(c: char) -> Option<Color> {
        match c {
            'g' => Some(Color::Green),
            'y' => Some(Color::Yellow),
            'r' => Some(Color::Gray),
            'w' => Some(Color::White),
            _ => None,
        }
    }
}


#[derive (Debug, Clone)]
pub struct ColoredLetter {
    letter: char,
    color: Color,
}
impl ColoredLetter {
    pub fn new(letter: char, color: Color) -> ColoredLetter {
       Self { letter, color, }
    }
    pub fn letter(&self) -> char {
        self.letter
    }
    pub fn color(&self) -> Color {
        self.color.clone()
    }
}

impl Eq for ColoredLetter {}
impl PartialEq for ColoredLetter {
    fn eq(&self, other: &ColoredLetter) -> bool {
        self.letter == other.letter && self.color == other.color
    }
}

impl Ord for ColoredLetter {
    fn cmp(&self, other: &ColoredLetter) -> Ordering {
        self.letter.cmp(&other.letter)
    }
}
impl PartialOrd for ColoredLetter {
    fn partial_cmp(&self, other: &ColoredLetter) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[derive (Debug, Clone)]
pub struct ColoredLetters {
    colored_letters: Vec<ColoredLetter>,
}

impl ColoredLetters {
    pub fn new() -> ColoredLetters {
        Self {colored_letters: Vec::new()}
    }

    pub fn from_str(guess: String, colors: String) -> ColoredLetters {
        let mut colored_letters = ColoredLetters::new();
        for (letter, color_letter) in guess.chars().zip(colors.chars()) {
            colored_letters.add(ColoredLetter::new(letter, Color::from_char(color_letter).unwrap()));
        }
        colored_letters
    }

    pub fn size(&self) -> usize {
        self.colored_letters.len()
    }

    pub fn letters(&self) -> Vec<ColoredLetter> {
        self.colored_letters.clone()
    }
    pub fn letters_with_color_count(&self, color: Color) -> usize {
        self.colored_letters
            .iter()
            .filter(|c| c.color() == color).count()
    }
    pub fn add(&mut self, colored_letter: ColoredLetter) {
        self.colored_letters.push(colored_letter);
    }
    pub fn letter_occurrences_with_color(&self, letter: char, color: Color) -> usize {
        self.colored_letters
            .iter()
            .filter(|c| c.letter() == letter && c.color() == color).count()
    }

    pub fn letter_occurrences(&self, letter: char) -> usize {
        self.colored_letters
            .iter()
            .filter(|c| c.letter() == letter).count()
    }

    pub fn replace_color(&mut self, ind:usize, color: Color) {
        self.colored_letters[ind].color = color;
    }
}

impl PartialEq for ColoredLetters {
    fn eq(&self, other: &ColoredLetters) -> bool {
        self.colored_letters == other.colored_letters
    }
}

impl Index<usize> for ColoredLetters {
    type Output = ColoredLetter;
    fn index(&self, index: usize) -> &Self::Output {
        &self.colored_letters[index]
    }
}

impl IntoIterator for ColoredLetters{
    type Item = ColoredLetter;
    type IntoIter = std::vec::IntoIter<ColoredLetter>;

    fn into_iter(self) -> Self::IntoIter {
        self.colored_letters.into_iter()
    }
}

#[derive (Debug, Clone)]
pub struct Rule {
    position: usize,
    colored_letter: ColoredLetter,
}
impl Rule {
    pub fn new(position: usize, letter: char, color: Color) -> Rule {
        let colored_letter = ColoredLetter::new(letter, color);
        Self::new_with_colored_letter(position, colored_letter)
    }
    pub fn new_with_colored_letter(position: usize, colored_letter: ColoredLetter) -> Rule {
        Self { position, colored_letter }
    }
    pub fn position(&self) -> usize {
        self.position
    }
    pub fn letter(&self) -> char {
        self.colored_letter.letter
    }
    pub fn color(&self) -> Color {
        self.colored_letter.color.clone()
    }
}

impl Eq for Rule {}
impl PartialEq for Rule {
   fn eq(&self, other: &Rule) -> bool {
       self.position == other.position && self.colored_letter == other.colored_letter
   }
}

impl Ord for Rule {
    fn cmp(&self, other: &Rule) -> Ordering {
        self.colored_letter.cmp(&other.colored_letter).then(self.position.cmp(&other.position))
    }
}
impl PartialOrd for Rule {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.colored_letter.cmp(&other.colored_letter).then(self.position.cmp(&other.position)))
    }
}

#[derive (Debug, Clone)]
pub struct Rules {
    rules: Vec<Rule>
}
impl Rules {
    pub fn new() -> Rules {
        Self {rules: Vec::new()}
    }
    pub fn size(&self) -> usize {
        self.rules.len()
    }
    pub fn empty(&self) -> bool {
        self.size() == 0
    }
    pub fn contains(&self, pos: usize, colored_letter: ColoredLetter) -> bool {
        let rule = Rule::new_with_colored_letter(pos, colored_letter);
        self.rules.contains(&rule)
    }
    pub fn add(&mut self, rule: Rule) {
        if rule.colored_letter.color == Color::Gray
            && self.letter_occurrences_with_color(rule.colored_letter.letter, Color::Gray) > 0 {
            return;
        }
        else if rule.colored_letter.color() == Color::Green
            && self.rules.contains(&rule) {
            return;
        }
        self.rules.push(rule);
    }

    pub fn rules(&self) -> Vec<Rule> {
        self.rules.clone()
    }

    pub fn letter_occurrences_with_color(&self, letter: char, color: Color) -> usize {
        self.rules
            .iter()
            .filter(|c| c.letter() == letter && c.color() == color).count()
    }
    pub fn colored_letter_occurrences(&self, colored_letter: &ColoredLetter) -> usize {
        self.rules
            .iter()
            .filter(|c| c.letter() == colored_letter.letter && c.color() == colored_letter.color).count()
    }

    pub fn letter_occurrences(&self, letter: char) -> usize {
        self.rules
            .iter()
            .filter(|c| c.letter() == letter).count()
    }

    pub fn color_occurrences(&self, color: Color) -> usize {
        self.rules
            .iter()
            .filter(|c| c.color() == color).count()
    }
}

impl Rules {
    pub fn all_green_used(&self, colored_letters: &ColoredLetters) -> bool {
        for rule in &self.rules {
            if rule.color() == Color::Green {
                if !(colored_letters[rule.position()].letter() == rule.letter()
                && colored_letters[rule.position()].color() == Color::Green) {
                    return false;
                }
            }
        }
        true
    }
    pub fn all_yellow_used(&self, colored_letters: &ColoredLetters) -> bool {
        for rule in &self.rules {
            if rule.color() == Color::Yellow {
                let letter = rule.letter().clone();
                if self.letter_occurrences_with_color(letter, Color::Yellow) >
                    (colored_letters.letter_occurrences_with_color(letter, Color::Yellow)
                    + colored_letters.letter_occurrences_with_color(letter, Color::Green)){
                    return false;
                }
            }
        }
        true
    }

    pub fn gray_used(&self, colored_letters: &ColoredLetters) -> bool {
        for rule in &self.rules {
            if rule.color() == Color::Gray {
                let letter = rule.letter().clone();
                if colored_letters.letter_occurrences(letter) >
                    self.letter_occurrences_with_color(letter, Color::Yellow) {
                    return true;
                }
            }
        }
        false
    }

    pub fn all_green_used_in_word(&self, word: &String) -> bool {
        for rule in &self.rules {
            if rule.color() == Color::Green {
                let letter = rule.letter().clone();
                if word.chars().nth(rule.position()).unwrap() != letter {
                    return false;
                }
            }
        }
        true
    }
    pub fn all_yellow_used_in_word(&self, word: &String) -> bool {
        for rule in &self.rules {
            if rule.color() == Color::Yellow {
                let letter = rule.letter().clone();
                if self.letter_occurrences_with_color(letter, Color::Yellow) >
                    word.chars().filter(|c| *c == letter).count() {
                    return false;
                }
            }
        }
        true
    }

    pub fn gray_used_in_word(&self, word: &String) -> bool {
        for rule in &self.rules {
            if rule.color() == Color::Gray {
                let letter = rule.letter().clone();
                if word.chars().filter(|c| *c == letter).count() >
                    self.letter_occurrences_with_color(letter, Color::Yellow) {
                    return true;
                }
            }
        }
        false
    }
}

impl PartialEq for Rules {
    fn eq(&self, other: &Rules) -> bool {
        let mut sorted_rules = self.rules.clone();
        let mut sorted_other = other.rules.clone();

        sorted_rules.sort();
        sorted_other.sort();

        sorted_rules.eq(&sorted_other)
    }
}

impl Index<usize> for Rules {
    type Output = Rule;
    fn index(&self, index: usize) -> &Self::Output {
        &self.rules[index]
    }
}
impl IntoIterator for Rules {
    type Item = Rule;
    type IntoIter = std::vec::IntoIter<Rule>;

    fn into_iter(self) -> Self::IntoIter {
        self.rules.into_iter()
    }
}