#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///

fn _to_base(number: u32, to_base: u32) -> Vec<u32> {
    let n_digits = (number as f32).log10() / (to_base as f32).log10();
    let n_digits = n_digits as u32 + 1;
    
    let mut remainder = number;
    let mut digits = Vec::<u32>::new();

    for index in 0 .. n_digits {
        let power = to_base.pow(n_digits - index - 1);
        let digit = remainder / power;
        digits.push(digit);
        remainder -= digit * power;
    }
    
    digits
}

fn _from_base(number: &[u32], base: u32) -> Result<u32, Error> {
    let mut sum: u32 = 0;
    for index in 0..number.len() {
        
        let value = number[index];
        if value >= base {
            return Err(Error::InvalidDigit(value));
        }
            
        let mult = base.pow((number.len() - index - 1) as u32);
        sum +=  mult * value;
    }
    
    Ok(sum)
}
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }

    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    if number.len() == 0 {
        return Ok(Vec::<u32>::from([0]));
    }


    let base_10 = match _from_base(number, from_base) {
        Err(e) => return Err(e),
        Ok(num) => num,
    };

    let encoded: Vec<u32> = _to_base(base_10, to_base);
    Ok(encoded)
}
