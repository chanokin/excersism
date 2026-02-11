pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut output = Vec::<String>::new();
    if digits.is_empty() {
        return output;
    }

    for idx in 0 .. digits.len() {
        if idx + len > digits.len() {
            break;
        }

        let end = idx+len;
        let chunk = digits.get(idx .. end).unwrap().to_string();
        output.push(chunk);
    }
    
    output
}
