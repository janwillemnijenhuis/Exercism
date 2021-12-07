pub fn production_rate_per_hour(speed: u8) -> f64 {
    let working_percentage = match speed {
        1..=4 => 1.00,
        5..=8 => 0.90,
        9..=10 => 0.77,
        _ => 0.00,
    };
    221.0 * (speed as f64) * working_percentage
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
