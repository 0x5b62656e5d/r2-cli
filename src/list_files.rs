use aws_sdk_s3::Client;
use aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Output;

pub async fn list_files(client: &Client, bucket: &str) -> Result<Vec<String>, anyhow::Error> {
    let res: ListObjectsV2Output = client.list_objects_v2().bucket(bucket).send().await?;

    Ok(res
        .contents
        .unwrap()
        .iter()
        .map(|o| o.key.clone().unwrap())
        .collect::<Vec<String>>())
}
