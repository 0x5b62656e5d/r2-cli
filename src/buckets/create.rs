use aws_sdk_s3::Client;

pub async fn create_bucket(client: &Client, name: String) -> Result<(), anyhow::Error> {
    client.create_bucket().bucket(name).send().await?;
    Ok(())
}
