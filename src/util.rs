use crate::client::config::{Regions, save_regions};
use aws_sdk_s3::Client;
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

pub async fn get_bucket_region(
    regions: &mut Regions,
    bucket: String,
    client: &Client,
) -> Result<String, anyhow::Error> {
    if let Some(region) = regions.buckets.get(bucket.as_str()) {
        Ok(region.clone())
    } else {
        let resp = client
            .get_bucket_location()
            .bucket(bucket.clone())
            .send()
            .await?;

        let region: String = match resp.location_constraint() {
            None => "us-east-1".to_string(),
            Some(aws_sdk_s3::types::BucketLocationConstraint::Eu) => "eu-west-1".to_string(),
            Some(other) => other.as_str().to_string(),
        };

        regions
            .buckets
            .insert(bucket.clone().to_string(), region.clone());

        save_regions(regions)?;

        Ok(region)
    }
}
