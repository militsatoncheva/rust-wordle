use std::cmp::Ordering;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Color {
    Green,
    Yellow,
    Gray,
    White,
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

impl PartialEq for ColoredLetter {
    fn eq(&self, other: &ColoredLetter) -> bool {
        self.letter == other.letter && self.color == other.color
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

    pub fn size(&self) -> usize {
        self.colored_letters.len()
    }

    pub fn letters(&self) -> Vec<ColoredLetter> {
        self.colored_letters.clone()
    }

    pub fn add(&mut self, colored_letter: ColoredLetter) {
        self.colored_letters.push(colored_letter);
    }
    pub fn letter_occurrences_with_color(&self, letter: char, color: Color) -> usize {
        let mut count = 0;
        for colored_letter in &self.colored_letters {
            if colored_letter.letter == letter
                && colored_letter.color == color {
                count += 1;
            }
        }
        count
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

#[derive (Debug, Clone)]
pub struct Rule {
    position: usize,
    letter: char,
    color: Color,
}
impl Rule {
    pub fn new(position: usize, letter: char, color: Color) -> Rule {
        Self { position, letter, color }
    }
    pub fn position(&self) -> usize {
        self.position
    }
    pub fn letter(&self) -> char {
        self.letter
    }
    pub fn color(&self) -> Color {
        self.color.clone()
    }
}

impl Eq for Rule {}
impl PartialEq for Rule {
   fn eq(&self, other: &Rule) -> bool {
       self.position == other.position && self.letter == other.letter && self.color == other.color
   }
}

impl Ord for Rule {
    fn cmp(&self, other: &Rule) -> Ordering {
        self.letter.cmp(&other.letter).then(self.color.cmp(&other.color))
    }
}
impl PartialOrd for Rule {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.letter.cmp(&other.letter).then(self.color.cmp(&other.color)))
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
    pub fn add(&mut self, rule: Rule) {
        if rule.color == Color::Gray
            && self.letter_occurrences_with_color(rule.letter, Color::Gray) > 0 {
            return;
        }
        else if rule.color == Color::Green
            && self.rules.contains(&rule) {
            return;
        }
        self.rules.push(rule);
    }

    pub fn rules(&self) -> Vec<Rule> {
        self.rules.clone()
    }

    pub fn letter_occurrences_with_color(&self, letter: char, color: Color) -> usize {
        let mut count = 0;
        for rule in &self.rules {
            if rule.letter == letter
                && rule.color == color {
                count += 1;
            }
        }
        count
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