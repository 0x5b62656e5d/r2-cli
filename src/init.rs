use crate::{
    config::{self, save_regions},
    s3_client::build_client,
    util::get_bucket_region,
};

pub async fn init_regions() -> Result<(), anyhow::Error> {
    let config: config::Config = config::get_config()?;
    let mut regions: config::Regions = config::get_regions()?;

    let default_client: aws_sdk_s3::Client =
        build_client(&config.default, "us-east-1".to_string()).await?;

    let buckets = default_client.list_buckets().send().await?;
    for b in buckets.buckets().iter() {
        let Some(name) = b.name() else {
            continue;
        };

        if regions.buckets.contains_key(name) {
            continue;
        }

        let region: String =
            get_bucket_region(&mut regions, name.to_string(), &default_client).await?;

        regions.buckets.insert(name.to_string(), region);
    }

    save_regions(&regions)?;

    Ok(())
}
