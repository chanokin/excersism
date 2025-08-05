pub fn reply(message: &str) -> &str {
    let all_whitespace: bool = message.split_whitespace().next() == None;
    
    if all_whitespace {
        return "Fine. Be that way!";
    }

    let has_alpha: bool = message.chars().any(char::is_alphabetic);
    let is_question: bool = message.trim().chars().last().unwrap() == '?';
    let is_all_caps: bool = message == message.to_uppercase() && has_alpha;

    if is_question {
        if is_all_caps {
            return "Calm down, I know what I'm doing!"
        }

        return "Sure.";
    }
    
    if is_all_caps {
        return "Whoa, chill out!";
    }
    
    
    "Whatever."
        
}
