pub fn number(user_number: &str) -> Option<String> {
    if user_number.len() < 10 {
        return None;
    }

    let mut clean = user_number.replace(".", "")
                               .replace("-", "");

    if clean.starts_with('+') {
        if let Some(first_space_index) = clean.find(char::is_whitespace) {
            let (start, end) = clean.split_at(first_space_index);
            let start = start.replace("+", "");
            
            if start.len() > 1 {
                return None;
            }

            if start != "1" {
                return None;
            }
            
            clean = end.trim().to_string();
        } else {
            return None;
        }
    }

    clean = clean.replace("(", "")
                 .replace(")", "")
                 .replace(char::is_whitespace, "");


    if clean.len() > 11 {
        return None;
    }

    if clean.len() == 11 {
        let (start, end) = clean.split_at(1);

        if start != "1" {
            return None;
        }

        clean = end.to_string();
    }
    
    for (index, ch) in clean.chars().enumerate() {
        if ! ch.is_numeric() {
            return None;
        }
        if (index == 0 || index == 3) && (ch == '0' || ch == '1') {
            return None;
        }

    }
    Some(clean.to_string())
    
}
