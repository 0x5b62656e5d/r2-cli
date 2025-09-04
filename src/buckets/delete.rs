use crate::client::config::{Regions, get_regions, save_regions};
use aws_sdk_s3::Client;

/// Deletes a S3 bucket and removes its entry from the local configuration
/// # Arguments
/// * `client` - A reference to the S3 client
/// * `name` - The name of the bucket
/// # Returns
/// * `Result<(), anyhow::Error>` - `Ok(())` if successful, error if the operation fails
pub async fn delete_bucket(client: &Client, name: String) -> Result<(), anyhow::Error> {
    client.delete_bucket().bucket(name.clone()).send().await?;

    let mut regions: Regions = get_regions()?;
    regions.buckets.remove(&name);
    save_regions(&regions)?;

    Ok(())
}
