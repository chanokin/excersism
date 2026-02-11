use std::iter::zip;

/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None
    }

    if s1.is_empty() {
        return Some(0);
    }
    
    // .collect::<Vec<_>>() => to force vec type
    Some(
        zip(s1.chars(), s2.chars()).filter(|(c0, c1)| c0 != c1)
                                   .collect::<Vec<_>>()
                                   .len()
    )
}
