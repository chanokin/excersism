pub fn eggCount(number: usize) usize {
    const n_bits: usize = @bitSizeOf(usize);
    const one: usize = 1;
    var mask: usize = 0;
    var n_eggs: u32 = 0;
    for (0..n_bits) |bit_index| {
        // use intCast to convert from uXX to uY here
        // because I do not know the XX due to target arch
        mask = one << @intCast(bit_index);
        n_eggs += @intFromBool((number & mask) > 0);
    }

    return n_eggs;
}
