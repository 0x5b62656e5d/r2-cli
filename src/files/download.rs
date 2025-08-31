use aws_sdk_s3::{Client, primitives::ByteStream};
use tokio::{
    fs::{File, create_dir_all},
    io::AsyncWriteExt,
};

pub async fn download_file(
    client: &Client,
    bucket: String,
    key: String,
    location: String,
    override_filename: Option<String>,
) -> Result<(), anyhow::Error> {
    let mut out = client
        .get_object()
        .bucket(bucket)
        .key(key.clone())
        .send()
        .await?;

    create_dir_all(location.clone()).await?;

    if override_filename.is_none() {
        write_to_file(&mut out.body, location, key).await?;
    } else {
        write_to_file(&mut out.body, location.clone(), override_filename.unwrap()).await?;
    }

    Ok(())
}

async fn write_to_file(
    bytestream: &mut ByteStream,
    location: String,
    filename: String,
) -> Result<(), anyhow::Error> {
    let mut file = File::create(format!(
        "{}/{}",
        location.replace("\"", "").trim_end_matches('/'),
        filename.replace("\"", "")
    ))
    .await?;

    while let Some(chunk) = bytestream
        .try_next()
        .await
        .map_err(|e| anyhow::anyhow!("S3 stream error: {e:?}"))?
    {
        file.write_all(&chunk).await?;
    }

    file.flush().await?;

    Ok(())
}
