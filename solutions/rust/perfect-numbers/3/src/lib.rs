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
        return Some(Classification::Deficient);
    }
    
    let aliquot: u64 = (1 .. num).filter(|fac| (num % fac) == 0).sum();

    match aliquot {
        x if x == num => Some(Classification::Perfect),
        x if num < x => Some(Classification::Abundant),
        _ => Some(Classification::Deficient)
    }

}
