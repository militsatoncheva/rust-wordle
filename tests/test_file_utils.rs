use std::fs::File;
use std::io::Write;
use tempfile::NamedTempFile;
use wordle::file_utils::{extract_words, choose_word};

#[test]
fn test_extract_word() {
    let content = "apple\nbanana\ncherry";
    let mut file = NamedTempFile::new().unwrap();
    file.write_all(content.as_bytes()).unwrap();
    let file_path = file.path().to_str().unwrap().to_string();
    let words = extract_words(file_path);

    assert_eq!(words, vec!["apple", "banana", "cherry"]);
}

#[test]
fn test_choose_word() {
    let words = vec![String::from("apple")
                     , String::from("banana"), String::from("cherry")];
    let word = choose_word(&words);
    assert!(words.contains(&word));
}