pub fn egg_count(display_value: u32) -> usize {
    let mut sum: usize = 0;
    for bit_index in 0..32 {
        let shifted: u32 = display_value >> bit_index;
        sum += (shifted & 1) as usize;
    }

    sum
}
