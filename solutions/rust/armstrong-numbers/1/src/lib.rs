pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 {
        return true;
    }
    
    let l10: u32 = num.ilog10();
    println!("log10 {num} = {l10}");
    let mut sum: u32 = 0;
    let mut residue: u32 = num;
    for power in 0..l10 {
        let power = l10 - power;
        let div = 10_u32.pow(power);
        let digit = residue / div;
        sum += digit.pow(l10 + 1);
        residue -= digit * div;
    }
    sum += residue.pow(l10 + 1);
    
    sum == num
}
