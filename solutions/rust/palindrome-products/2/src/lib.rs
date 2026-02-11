use std::collections::{HashSet, BTreeMap,};

#[derive(Debug, Clone)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl PartialEq for Palindrome {
    fn eq(&self, other: &Palindrome) -> bool {
        self.value() == other.value()
    }
}

impl Eq for Palindrome {}

impl Palindrome {
    pub fn new(value: u64) -> Self {
        Self{
            value,
            factors: HashSet::new(),
        }
    }
    
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(&self) -> HashSet<(u64, u64)> {
        self.factors.clone()
    }

    pub fn add_factors(&mut self, factors: (u64, u64)) {
        self.factors.insert(factors);
    }
}



pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if max < min {
        return None;
    }
    if max - min < 2 {
        return None;
    }

    fn is_palindrome(num: u64) -> bool {
        let s = num.to_string();
        s == s.chars().rev().collect::<String>()
    }
    
    let mut pals: BTreeMap<u64, Palindrome> = BTreeMap::new();
    for f0 in min .. max + 1 {
        for f1 in f0 .. max + 1 {

            let c = f0 * f1;
            if is_palindrome(c) {

                if pals.contains_key(&c) {
                    let p = pals.get_mut(&c).unwrap();
                    p.add_factors((f0, f1));
                    
                } else {
                    let mut p = Palindrome::new(c);
                    p.add_factors((f0, f1));
                    pals.insert(c, p);
                }
                
            }
        }
    }

    let (_, smallest) = pals.first_key_value().unwrap();
    let (_, largest) = pals.last_key_value().unwrap();
    
    Some((smallest.clone(), largest.clone()))
}
