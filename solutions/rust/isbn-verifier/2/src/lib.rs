/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    fn to_number(c: char) -> u32 {
        if c == 'X' {
            return 10;
        }

        c.to_digit(10)
         .expect("can't convert this digit")
    }

    if isbn.len() < 10 {
        return false;
    }

    let max_idx = isbn.len() - 1;
    let mut char_pos: u32 = 0;
    let mut check: u32 = 0;
    for (index, c) in isbn.chars().enumerate() {
        if c == '-' {
            continue;
        }
        
        if index < max_idx 
           && !c.is_numeric() {
            
            return false;
        }

        if index == max_idx
           && !(c.is_numeric() || c == 'X') {
            
            return false;
        }

        check += (10 - char_pos) * to_number(c);
        char_pos += 1;
    }

    if char_pos != 10 {
        return false;
    }
    
    check % 11 == 0
}
