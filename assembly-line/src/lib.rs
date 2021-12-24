const CARS_PRODUCED_PER_HOUR: u32 = 221;

#[inline]
pub fn production_rate_per_hour(speed: u8) -> f64 {
    let rate = match speed {
        0 => 0.0,
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => unreachable!(),
    };
    (CARS_PRODUCED_PER_HOUR * speed as u32) as f64 * rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
