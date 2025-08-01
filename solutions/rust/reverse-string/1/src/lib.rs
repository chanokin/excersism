pub fn reverse(input: &str) -> String {
    let size: usize = input.len();
    let mut ret: String = String::with_capacity(size);
    if size == 0 {
        return ret;
    }
    let mut chrs_iterator = input.chars();
    for idx in 0..size {
        let chr_opt = chrs_iterator.next_back();
        
        if chr_opt.is_none() {
            break;
        }
        
        ret.push(chr_opt.unwrap());
    }
    ret 
}
