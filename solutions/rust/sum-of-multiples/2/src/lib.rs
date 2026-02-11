use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    if factors.is_empty() {
        return 0_u32;
    }
    
    let mut multiples = HashSet::<u32>::new();

    for factor in factors {
        if *factor == 0 {
            continue;
        }
        
        let mut multiple: u32 = *factor;
        while multiple < limit {
            multiples.insert(multiple);
            multiple += *factor;
        }
        
    }


    multiples.into_iter().sum()
}
