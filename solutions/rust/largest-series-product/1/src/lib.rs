#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    // println!("{string_digits} of len {} with span {}", string_digits.len(), span);
    if span == 0 {
        return Ok(1);
    }

    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }
    
    let mut digits: Vec<u64> = Vec::new();
    for ch in string_digits.chars() {
        if let Some(digit) = ch.to_digit(10) {
            digits.push(digit.into());
        } else {
            return Err(Error::InvalidDigit(ch));
        }
    }


    let mut max_prod: u64 = 0;
    for start in 0..digits.len() - span + 1 {
        let end = start + span;
        
        let prod:u64 = digits.get(start..end).unwrap().into_iter().product();

        if prod > max_prod {
            max_prod = prod;
        }

        // println!(
        //     "{} => {}:\n\t{:?} prod = {} ::: max {}",
        //     start,
        //     end,
        //     digits.get(start..end),
        //     prod,
        //     max_prod,
        // );
        
    }

    
    

    Ok(max_prod)
}
