pub fn encode(source: &str) -> String {
    if source.is_empty() {
        return source.to_string();
    }

    // storing is not needed but helps to debug
    let mut order: Vec<(char, u32)> = Vec::new();
    let mut last_ch: char = source.chars().next().unwrap();
    let mut count: u32 = 0;
    for ch in source.chars() {
        if ch == last_ch {
            count += 1;
        } else {
            order.push((last_ch, count));
            last_ch = ch;
            count = 1;
        }
    }
    
    order.push((last_ch, count));

    
    let mut output: String = String::new();
    for (ch, num) in order {
        // println!("{ch} = {num}");
        let enc = if num == 1 {ch.to_string()} else {format!("{num}{ch}")};
        output.push_str(&enc);
    }

    output
}

pub fn decode(source: &str) -> String {
    if source.is_empty() {
        return source.to_string();
    }
    
    let mut output: String = String::new();
    let character_positions: Vec<(usize, char)> = source.char_indices()
                                                        .filter(|(idx, ch)| ch.is_alphabetic() || *ch == ' ')
                                                        .collect();
    let mut start_idx: usize = 0;    
    for (idx, ch) in character_positions {
        
        // get substring from start_idx -> idx
        if idx - start_idx == 0 {
            // this means it's a single character (not repeated)
            output.push(ch);
        }        
        else if let Result::Ok(repetitions) = source.get(start_idx..idx).expect("wrong range").parse::<usize>() {
            output.push_str(ch.to_string().repeat(repetitions).as_str());
        }

        // jump over current character
        start_idx = idx + 1;
    }
    
    output
}
