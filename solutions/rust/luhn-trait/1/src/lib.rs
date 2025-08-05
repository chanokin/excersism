pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

pub fn is_valid(code: &str) -> bool {
    let mut _code = Vec::<u32>::new();
    let mut index: usize = 0;
    for ch in code.chars().rev() {
        let _digit = ch.to_digit(10);
        if _digit.is_none() {
            if ch != ' ' {
                return false;
            }
            
            continue;
        }
        
        let mut digit: u32 = _digit.unwrap();
        if (index % 2) != 0 {
            digit *= 2;
            if digit > 9 {
                digit -= 9;
            }
        }
        println!("digit {digit}");
        _code.push(digit);
        index += 1;
    }
    
    let sum: u32 = _code.iter().sum();
    println!("sum {sum}");

    // index here is 1 iif there was a single 0 and N spaces in the string
    if sum == 0 && index == 1 {
        return false;
    }
    
    (sum % 10) == 0
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        is_valid(&self.to_string())
    }
}
