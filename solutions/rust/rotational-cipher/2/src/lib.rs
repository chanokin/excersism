pub fn rotate(input: &str, key: u8) -> String {
    let mut output = String::from("");

    for ch in input.chars() {
        if ! ch.is_alphabetic() {
            output.push(ch);
            continue;
        }
        let base_u8 = if ch.is_lowercase() {97} else {65};
        let rotated = ((ch as u8) - base_u8 + key) % 26;
        output.push((rotated + base_u8) as char)
    }
    
    output
}
