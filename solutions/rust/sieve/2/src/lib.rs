use std::iter::zip;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {

    if upper_bound == 1 {
        return Vec::new();
    }

    let values = Vec::<u64>::from_iter(2..upper_bound+1); 
    let mut is_prime: Vec<bool> = values.iter().map(|_x| true).collect();

    for (index, val) in values.iter().enumerate() {
        if index + 1 < values.len() {
            for (test_index, test_val) in values.iter().enumerate() {
                if test_index <= index {
                    continue;
                }
                if let Some(prime) = is_prime.get_mut(test_index){
                    if *prime && test_val % val == 0 {
                        *prime = false;
                    }
                }
                    
            }
        }
        
    }

    zip(values, is_prime).filter(|(_v, p)| *p).map(|(v, _p)| v).collect()

}
