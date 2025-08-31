use crate::config::{Regions, get_regions, save_regions};
use aws_sdk_s3::{
    Client,
    types::{BucketLocationConstraint, CreateBucketConfiguration},
};

pub async fn create_bucket(
    client: &Client,
    name: String,
    region: String,
) -> Result<(), anyhow::Error> {
    client
        .create_bucket()
        .bucket(name.clone())
        .create_bucket_configuration(
            CreateBucketConfiguration::builder()
                .location_constraint(BucketLocationConstraint::from(region.clone().as_str()))
                .build(),
        )
        .send()
        .await?;

    let mut regions: Regions = get_regions()?;
    regions.buckets.insert(name.clone(), region.clone());
    save_regions(&regions)?;

    Ok(())
}
