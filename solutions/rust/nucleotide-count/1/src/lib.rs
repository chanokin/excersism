use std::collections::HashMap;

const VALID_NUCLEOTIDES: [char; 4] = ['G', 'A', 'T', 'C'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !VALID_NUCLEOTIDES.contains(&nucleotide) {
        return Err(nucleotide);
    }
    
    let mut size: usize = 0;
    for c in dna.chars() {
        if c == nucleotide {
            size += 1;
        } else if !VALID_NUCLEOTIDES.contains(&c) {
            return Err(c);
        }
    }

    Ok(size)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut res: HashMap<char, usize> = HashMap::new();
    for nucleotide in VALID_NUCLEOTIDES {
        res.insert(
            nucleotide, 
            count(nucleotide, dna)? // this returns if error
        );
    }
    Ok(res)
}
