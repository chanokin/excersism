#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    dna: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    rna: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (index, ch) in dna.chars().enumerate() {
            if ! ['G', 'A', 'T', 'C'].contains(&ch) {
                return Err(index);
            }
        }
        
        Ok(Self {
            dna: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> Rna {
        let mut rna: Vec<String> = Vec::new();
        for ch in self.dna.chars() {
            let converted = match ch {
                'G' => "C",
                'C' => "G",
                'T' => "A",
                 _  => "U", // A -> U --- already filtered/valid dna
            };
            rna.push(converted.to_string());
        }
        Rna::new(rna.join("").as_str()).expect("needed valid dna")
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (index, ch) in rna.chars().enumerate() {
            if ! ['G', 'A', 'U', 'C'].contains(&ch) {
                return Err(index);
            }
        }

        Ok(Self {
            rna: rna.to_string(),
        })
        
    }
}
