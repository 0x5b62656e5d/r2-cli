use aws_sdk_s3::{Client, primitives::ByteStream};
use std::fs;

pub async fn upload_file(
    client: &Client,
    bucket: String,
    key: String,
    file_path: String,
) -> Result<(), anyhow::Error> {
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from(fs::read(file_path)?))
        .send()
        .await?;

    Ok(())
}
