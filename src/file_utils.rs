use std::fs::File;
use std::io::Read;
use rand::Rng;

pub fn extract_words(file_name: String) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut words = String::new();
    file.read_to_string(&mut words).unwrap();
    words.lines().map(|x| x.to_string()).collect()
}

pub fn choose_word(words:&Vec<String>) -> String {
    let mut rng= rand::rng();
    let number = rng.random_range(1..words.len());
    words[number].clone()
}