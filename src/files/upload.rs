use aws_sdk_s3::{Client, primitives::ByteStream};
use std::fs;
use tree_magic::from_u8;

/// Uploads a file to an S3 bucket at the specified key (path).
/// # Arguments
/// * `client` - A reference to the S3 client
/// * `bucket` - The name of the bucket
/// * `key` - The key (path) where the file will be uploaded
/// * `file_path` - The local path of the file to upload
/// # Returns
/// * `Result<(), anyhow::Error>` - `Ok(())` if successful, error if the operation fails
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
