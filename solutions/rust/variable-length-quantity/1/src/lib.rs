#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

fn single_val_to_bytes(v: u32) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    
    let base: u32 = 128;
    let n_bytes = (v as f64).log10() / (base as f64).log10();
    let n_bytes = 1 + n_bytes.floor() as u32;
    
    let mut residue: u32 = v;
    for byte_index in (0..n_bytes).rev() {
        let divisor = base.pow(byte_index);
        let div = residue / divisor;

        residue -= div * divisor;

        // 0x80 = 0b1000_0000 
        let div = div as u8;
        let div = if byte_index > 0 {0x80 | div} else {div};

        out.push(div);
    }

    out
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    for val in values {
        for byte in single_val_to_bytes(*val) {
            output.push(byte);
        }
        
    }

    output
}


fn decode_number(bytes_for_number: &[u32]) -> u32 {
    let mut output: u32 = 0;
    let n_bytes: u32 = bytes_for_number.len() as u32;
    // println!("n_bytes {n_bytes}");
    let base: u32 = 128;
    
    for byte_index in (0..n_bytes).rev() {
        let byte = bytes_for_number[byte_index as usize];
        let base_multiplier = base.pow(byte_index) ;
        // println!("\t\tbyte {byte} * {base_multiplier} = {}", byte * base_multiplier);
        output += base_multiplier * byte;
    }
    // println!("\t\t\t{output}");
    output
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut last_seq_bit_zero = bytes.len() > 1;
    let mut output: Vec<u32> = Vec::new();
    let mut per_number_bytes: Vec<u32> = Vec::new();
    for byte in bytes {
        let seq_bit = 0x80 & byte;
        // println!("byte {byte} seq_bit {}", seq_bit > 0);

        if last_seq_bit_zero && seq_bit == 0 {
            if *byte == 0 {
                output.push(0);
                continue;
            }

            return Err(Error::IncompleteNumber);
        }

        last_seq_bit_zero = seq_bit == 0;
        
        let byte = 0x7F & byte;
        // println!("\tbyte {}", byte);

        per_number_bytes.insert(0, byte as u32);

        if last_seq_bit_zero {
            output.push(decode_number(&per_number_bytes));
            per_number_bytes.clear();
        }
    }

    if output.is_empty() {
        return Err(Error::IncompleteNumber);
    }
    Ok(output)
}
