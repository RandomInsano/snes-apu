pub fn multiply_volume(value: i32, volume: u8) -> i32 {
    (value * (volume as i32)) >> 7
}

pub fn clamp(value: i32) -> i32 {
    value.clamp(-32768, 32767)
}
