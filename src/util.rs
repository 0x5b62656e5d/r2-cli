use crate::config::Regions;
use tabled::{
    Table, Tabled,
    settings::{Color, Modify, Style, object::Rows},
};

pub fn round(value: f64, precision: u32) -> f64 {
    let factor: f64 = 10f64.powi(precision as i32);

    (value * factor).round() / factor
}

pub fn build_table<A, B, C, D>(data: A, map_fn: B) -> Table
where
    A: IntoIterator<Item = C>,
    B: Fn(&C) -> D,
    D: Tabled,
{
    Table::new(
        data.into_iter()
            .map(|item: C| map_fn(&item))
            .collect::<Vec<D>>(),
    )
    .with(Style::modern())
    .with(Modify::new(Rows::first()).with(Color::FG_BRIGHT_MAGENTA))
    .to_owned()
}

pub fn get_bucket_region(regions: &Regions, bucket: String) -> String {
    regions
        .buckets
        .get(bucket.as_str())
        .cloned()
        .unwrap_or_default()
}
