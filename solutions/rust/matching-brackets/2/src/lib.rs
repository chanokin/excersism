pub fn brackets_are_balanced(string: &str) -> bool {
    if string.is_empty() {
        return true;
    }

    let oppening = "({[";
    let closing = "]})";
    let mut brackets = String::new(); // stack
    for ch in string.chars() {
        // println!("char {ch} : brackets {brackets}");
        if oppening.chars().any(|x| x == ch) {
            brackets.push(ch);
            // println!("found oppening {ch}");
            continue;
        } 

        // not a bracket - don't add to stack
        if ! closing.chars().any(|x| x == ch) {
            continue;
        }

        // after the last if - we are sure ch is a closing bracket

        // if we start adding with a closing bracket, return false
        if brackets.is_empty() {
            return false;
        }
        
        let last_ch = brackets.chars().last().unwrap();
        
        if ch == ')' && last_ch == '(' {
            // println!("found pair ()");
            brackets.pop();
        
        } else if ch == ']' && last_ch == '[' {
            // println!("found pair []");
            brackets.pop();

        } else if ch == '}' && last_ch == '{' {
            // println!("found pair {{}}");
            brackets.pop();
            
        } else {
            // as soon as we miss a matching pair exit
            return false;
        }
    }

    // if we reached here and we didn't clean everything,
    // stack is not empty, then we didn't pair every bracket
    // println!("brackets {brackets}");
    brackets.is_empty()
}
