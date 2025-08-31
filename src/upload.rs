use aws_sdk_s3::{Client, primitives::ByteStream};
use std::fs;
use tree_magic::from_u8;

pub async fn upload_file(
    client: &Client,
    bucket: String,
    key: String,
    file_path: String,
) -> Result<(), anyhow::Error> {
    let bytes: ByteStream = ByteStream::from(fs::read(file_path)?);

    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .content_type(from_u8(bytes.bytes().unwrap()))
        .body(bytes)
        .send()
        .await?;

    Ok(())
}
