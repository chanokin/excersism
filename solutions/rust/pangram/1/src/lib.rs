use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    if sentence.is_empty() {
        return false;
    }

    let sentence = sentence.to_lowercase();
    let mut split_sentence: Vec<&str> = sentence
        .split(&['_', ' ', '"', '.'][..])
        .filter(|x| x.chars().all(char::is_alphabetic))
        .collect();

    let mut alphabet: HashSet<char> = HashSet::new();
    for word in split_sentence {
        // println!("{word}");
        for ch in word.chars() {
            alphabet.insert(ch);
        }
    }
    
    
    alphabet.len() == 26
}
