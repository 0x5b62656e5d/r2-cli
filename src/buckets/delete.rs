use crate::config::{Regions, get_regions, save_regions};
use aws_sdk_s3::Client;

pub async fn delete_bucket(client: &Client, name: String) -> Result<(), anyhow::Error> {
    client.delete_bucket().bucket(name.clone()).send().await?;

    let mut regions: Regions = get_regions()?;
    regions.buckets.remove(&name);
    save_regions(&regions)?;

    Ok(())
}
