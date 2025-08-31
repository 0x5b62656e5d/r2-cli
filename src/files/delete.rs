use aws_sdk_s3::Client;

pub async fn delete_file(
    client: &Client,
    bucket: String,
    key: String,
) -> Result<(), anyhow::Error> {
    client
        .delete_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await?;

    Ok(())
}
