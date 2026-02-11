pub fn factors(n: u64) -> Vec<u64> {
    let mut _factors: Vec<u64> = Vec::new();
    if n == 1 { return _factors; }
    // todo!("This should calculate the prime factors of {n}")
    let mut remainder: u64 = n;
    let mut prime: u64 = 2;
    let max_loops = 1000000;
    let mut loop_index = 0;
    loop {
        if remainder == 1 { break; }
        
        let is_divisible = remainder % prime == 0;
        
        if is_divisible {
            _factors.push(prime);
            remainder /= prime;
        } else {
            prime += 1;
        }
        
        loop_index += 1;
        if loop_index == max_loops { println!("MAX"); break; }
    }
    _factors  
}
