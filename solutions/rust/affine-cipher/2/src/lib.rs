use num::integer::gcd;
use num::bigint::BigInt;
use num::cast::ToPrimitive;
use std::cmp::min;




/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let _xgcd = gcd(a, 26);
    // println!("gcd: {a}, {b} = {_xgcd}");
    if _xgcd > 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }
    
    // println!("byte val of a {} and A {}", 'a' as u8, 'A' as u8 );

    let mut encoded_words: Vec<String> = Vec::new();
    for word in plaintext.split_whitespace() {
        let mut encoded = String::new();
        for ch in word.chars() {
            
            if ch.is_numeric() {
                encoded.push(ch);
                
            } 
            else if ch.is_alphabetic() {
                let ch = ch.to_lowercase().next().unwrap(); 
                let mut num: i32 = (ch as u8).into();
                num -= 97;
                let x = (a * num + b) % 26;
                let enc = char::from((x + 97)as u8);
                // println!("{ch} => {num} => {x} => {enc}");
                encoded.push(enc);
            }
            
        }
        encoded_words.push(encoded);
    }

    // println!("{encoded_words:?}");
    
    let encoded = encoded_words.join("");
    // println!("{encoded}");

    encoded_words.clear();
    for start in (0..encoded.len()).step_by(5) {
        let end = min(start + 5, encoded.len());
        encoded_words.push(encoded[start..end].to_string());
    }
    
    // println!("{encoded_words:?}");
    Ok(encoded_words.join(" "))
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let _xgcd = gcd(a, 26);
    // println!("gcd: {a}, {b} = {_xgcd}");
    if _xgcd > 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }
    // the modular multiplicative inverse only exists if a and m [26] are coprime.
    let modinv = BigInt::from(a).modinv(&BigInt::from(26))
                                    .unwrap()
                                    .to_i32()
                                    .expect("should be able to cast to i32");
    
    // println!("modinv {}", modinv);
    let ciphertext = ciphertext.replace(" ", "");
    // println!("no spaces {ciphertext}");
    let mut decoded = String::new();
    for ch in ciphertext.chars() {

        if ch.is_numeric() {
            decoded.push(ch);
        } 
        else if ch.is_alphabetic() {
            let ch = ch.to_lowercase().next().unwrap(); 
            let mut num: i32 = (ch as u8).into();
            num -= 97;
            
            
            // println!("{ch} => {num} ");
            let x:i32 = modinv * (num - b);
            let x = x.rem_euclid(26);
            let enc = char::from((x + 97) as u8);
            // println!("{ch} => {num} => {x} => {enc}");
            decoded.push(enc);
        }

    }
    
    Ok(decoded)
}
