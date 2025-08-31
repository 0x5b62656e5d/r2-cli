use aws_sdk_s3::Client;

pub async fn delete_bucket(client: &Client, name: String) -> Result<(), anyhow::Error> {
    client.delete_bucket().bucket(name).send().await?;
    Ok(())
}
