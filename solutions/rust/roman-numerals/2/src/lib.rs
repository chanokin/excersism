use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    number: String,
}

impl Roman {
    pub fn new(value: &str) -> Self {
        Self {
            number: String::from(value),
        }
    }
}

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        write!(_f, "{}", self.number)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let numbers = [1000, 900, 500, 400, 
                       100,  90,  50,  40, 
                       10,   9,   5,   4, 
                       1];
        let letters = ["M", "CM", "D", "CD",
                       "C", "XC", "L", "XL",
                       "X", "IX", "V", "IV",
                       "I"];
        
        let mut num = num as i32; // just allow us to decrease its value
        let mut output = String::from("");
        for (index, value) in numbers.iter().enumerate() {
            if let Some(numeral) = letters.get(index) {
                while num >= *value {
                    num -= value;
                    output.push_str(numeral);
                }
            }
        }
        
        Roman::new(output.as_str())
    }
}
