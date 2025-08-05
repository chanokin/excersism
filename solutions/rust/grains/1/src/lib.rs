pub fn square(s: u32) -> u64 {
    // todo!("grains of rice on square {s}");
    2_u64.pow(s-1)
}

pub fn total() -> u64 {
    (1..65).map(|x| square(x)).sum()
    // todo!();
}
