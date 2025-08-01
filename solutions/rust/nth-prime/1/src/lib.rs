pub fn nth(n: u32) -> u32 {
    let un: usize = n as usize;
    let mut primes: Vec<u32> = Vec::from([2, 3, 5, 7, 11, 13]);
    if un < primes.len() { return *primes.get(un).unwrap(); }

    let mut i = 14;
    
    loop  {
        let mut is_prime = true;
        // see if this current number (i) is exactly divisible by any prime
        for prime in &primes {
            if (i % prime) == 0 {
                is_prime = false;
                break;
            }
        }
        
        if is_prime {
            primes.push(i);
        }
        
        if primes.len() == un + 1 {
            break;
        }
        
        i += 1;
    }

    *primes.last().unwrap()
}

