fn translate(plain: &str) -> String {
    let trans = "zyxwvutsrqponmlkjihgfedcba";

    let mut output = String::new();
    // println!("length of trans dict {}", trans.len());
    for ch in plain.chars() {
        if ch.is_alphabetic() {
            let mut ch_u8 = ch.to_lowercase().next().unwrap() as u8;
            // print!("{ch} => {ch_u8} ");
            ch_u8 -= 97;
            // print!("=> {ch_u8}");

            if let Some(translated) = trans.chars().nth(ch_u8 as usize) {
                // println!(" => {translated}");
                output.push(translated);
            } 
            // else {
            //     println!(" => couldn't get char at index {ch_u8}");
            // }
            
                
        } else if ch.is_numeric() {
            output.push(ch);
        }
        
    }

    output
}
/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let no_spaces = translate(plain);
    let mut output_with_spaces: Vec<&str> = Vec::new();
    for start in (0..no_spaces.len()).step_by(5) {
        let end = std::cmp::min(start + 5, no_spaces.len());
        output_with_spaces.push(no_spaces.get(start..end).unwrap());
    }

    output_with_spaces.join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    translate(cipher)
}
