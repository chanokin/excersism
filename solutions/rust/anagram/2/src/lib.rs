use std::collections::HashSet;
use std::collections::HashMap;
// added 'a to possible anagrams to make it compatible with return type
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {

    let _word = String::from(word).to_lowercase();
    let mut counts_input = HashMap::new();
    for ch in _word.chars() {
        if let Some(val) = counts_input.get_mut(&ch) {
            *val += 1;
        } else {
            counts_input.insert(ch, 1);
        }
    }

    
    let mut accepted = HashSet::<&'a str>::new();
    for candidate in possible_anagrams {
        // dereference as list is a ref plus each elem is a ref
        let _candidate = String::from(*candidate).to_lowercase();

        // same word is not an anagram of itself - cheeky
        if _word == _candidate {
            continue;
        }
        
        let mut counts_candidate = HashMap::new();
        for ch in _candidate.chars() {
            if let Some(val) = counts_candidate.get_mut(&ch) {
                *val += 1;
            } else {
                counts_candidate.insert(ch, 1);
            }
        }

        if counts_candidate.len() != counts_input.len() {
            continue;
        }
        
        let mut discard_candidate: bool = false;
        for (ch, count) in counts_input.iter() {
            if let Some(candidate_count) = counts_candidate.get(ch) {
                if *count != *candidate_count {
                    discard_candidate = true;
                    break;
                }
            } else {
                discard_candidate = true;
                break;
            }
        }

        if discard_candidate {
            continue;
        }
        
        accepted.insert(candidate);
     }

    accepted
}
