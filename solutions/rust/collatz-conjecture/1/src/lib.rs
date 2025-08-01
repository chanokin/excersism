pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    
    let mut x: u64 = n;
    let mut steps: u64 = 0;
    loop {
        if x == 1 {
            break;
        }

        if x % 2 == 0 {
            x /= 2;
            
        } else {
            x = 3 * x + 1;
        }

        steps += 1;
    }

    Some(steps)
}
