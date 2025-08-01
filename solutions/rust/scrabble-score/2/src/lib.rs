/// Compute the Scrabble score for a word.
fn to_val(ch: char) -> u64 {
    let output: u64 = match ch {
        'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => 1,
        'd' | 'g' => 2,
        'b' | 'c' | 'm' | 'p' => 3,
        'f' | 'h' | 'v' | 'w' | 'y' => 4,
        'k' => 5,
        'j' | 'x' => 8,
        'q' | 'z' => 10,
        _ => 0,
    };

    output
}

pub fn score(word: &str) -> u64 {
    word.to_lowercase()
        .chars()
        .map(|ch| to_val(ch))
        .collect::<Vec<u64>>()
        .iter()
        .sum()
}
