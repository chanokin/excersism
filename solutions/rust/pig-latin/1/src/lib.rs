fn rule_1(input: &str) -> Option<String> {
    
    if input.starts_with(&['a', 'e', 'i', 'o', 'u'])
       || input.starts_with("xr")
       || input.starts_with("yt")
       || input.starts_with("ay") {
        return Some(input.to_owned() + "ay");
    }
    
    None
}


fn split_join(split_index:usize, input: &str) -> String {
    let (start, end) = input.split_at(split_index);

    println!("{start} {end}");
    end.to_owned() + start + "ay"
}


fn rules_234(input: &str) -> Option<String> {
    let first_vowel_index = input.find(&['a', 'e', 'i', 'o', 'u']);
    
    if first_vowel_index.is_none() {
        // no vowel but could still have a "y"
        let first_y = input.find("y");
        if first_y.is_none() {
            return None;
        }

        return Some(split_join(first_y.unwrap(), input));
    }

    let first_vowel_index = first_vowel_index.unwrap();

    if first_vowel_index == 0 {
        // we should never reach this condition!
        return None;
    }
    
    let (start, end) = input.split_at(first_vowel_index);

    let first_q = start.find("q");
    if first_q.is_some() && end.starts_with("u") {
        return Some(split_join(first_q.unwrap() + 2, input));
    }
    
    Some(end.to_owned() + start + "ay")
}

pub fn translate(input: &str) -> String {
    if input.is_empty() {
        return input.to_string();
    }

    let words: Vec<&str> = input.split(' ').collect();
    let mut out_vec: Vec<String> = Vec::new();
    
    for word in words {
        let _word = rule_1(word);
        if _word.is_some() {
            out_vec.push(_word.unwrap());
            continue;
        }

        
        let word = rules_234(word);
        if word.is_some() {
            out_vec.push(word.unwrap());
        }
    }


    out_vec.join(" ")
}
