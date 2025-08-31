pub fn round(value: f64, precision: u32) -> f64 {
    let factor: f64 = 10f64.powi(precision as i32);

    (value * factor).round() / factor
}
