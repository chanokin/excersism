use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    if factors.is_empty() {
        return 0_u32;
    }

    if factors.len() == 1 && *factors.first().unwrap() == 0 {
        return 0_u32;
    }
    
    let mut multiples = HashSet::<u32>::new();

    for factor in factors {
        let mut multiple: u32 = *factor;
        let while_limit = 5000;
        let mut while_count = 1;
        // println!("\nfactor {factor}");
        while multiple < limit {
            multiples.insert(multiple);
            multiple += *factor;

            // println!("multiple {multiple}");
            if while_count > while_limit {
                break;
            }
            while_count += 1;
        }
        
    }


    multiples.into_iter().sum()
}
