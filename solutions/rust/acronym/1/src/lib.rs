pub fn abbreviate(phrase: &str) -> String {
    let phrase = phrase.replace(".", "").replace("_", "");
    // todo!("Given the phrase '{phrase}', return its acronym");
    let splitted: Vec<&str> = phrase.split(
                                |c: char| 
                                    c == '-' 
                                    || c.is_whitespace()
                              ).collect();

    let mut acronym = String::new();
    
    for v in splitted {
        println!("word {v}");

        let n_uppercase = v.chars().filter(|c| c.is_uppercase()).count();
        let is_camelcase = n_uppercase > 1 && n_uppercase < v.len();
        if is_camelcase {
            let chars = v.chars().filter(|c| c.is_uppercase());
            for ch in chars {
                acronym.push(ch);
            }
        }
        else {
            let ch = v.chars().next();
            if ch.is_none() {
                continue;
            }
            acronym.push_str(&ch.unwrap().to_uppercase().to_string());
        }        
    }
    
    
    acronym
}
