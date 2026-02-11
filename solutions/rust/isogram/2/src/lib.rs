use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let cleaned: Vec<char> = candidate.replace(" ", "")
                                      .replace("-", "")
                                      .to_lowercase()
                                      .chars()
                                      .collect();
    let original_len = cleaned.len();
    let cleaned: HashSet<char> = HashSet::from_iter(
                                    cleaned.iter().cloned());
    
    original_len == cleaned.len()
}
