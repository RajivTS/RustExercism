const PER_HOUR_PRODUCTION: u32 = 221;
const MINUTES_IN_HOUR: u32 = 60;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let success_rate: f64 = match speed {
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => 0.0
    };
    return speed as f64 * PER_HOUR_PRODUCTION as f64 * success_rate;
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    return production_rate_per_hour(speed) as u32 / MINUTES_IN_HOUR;
}
