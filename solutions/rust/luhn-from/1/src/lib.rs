pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn new(code: String) -> Self {
        Self {
            code,
        }
    }
    
    pub fn is_valid(&self) -> bool {
        let mut _code = Vec::<u32>::new();
        let mut index: usize = 0;
    
        for ch in self.code.chars().rev() {
            if let Some(mut digit) = ch.to_digit(10) {
                if (index % 2) != 0 {
                    digit *= 2;
        
                    if digit > 9 {
                        digit -= 9;
                    }
                }
                // println!("digit {digit}");
                _code.push(digit);
                index += 1;
            }
            else {
                // if the character we failed to convert to digit is not
                // a space, we should just say this code is invalid
                if ch != ' ' {
                    return false;
                }
        
                continue;
            }
    
        }
    
        let sum: u32 = _code.iter().sum();
    
        // println!("sum {sum}");
    
        // index here is 1 iif there was a single 0 and N spaces in the string
        if sum == 0 && index == 1 {
            return false;
        }
    
        (sum % 10) == 0
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Self {
            code: input.to_string(),
        }
    }


}
