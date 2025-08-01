#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    if num == 1 {
        return return Some(Classification::Deficient);
    }
    
    let aliquot: u64 = (1 .. num).filter(|fac| (num % fac) == 0).sum();

    if aliquot  == num {
        return Some(Classification::Perfect);
    } else if num < aliquot {
        return Some(Classification::Abundant);
    } else {
        return Some(Classification::Deficient);
    }
}
