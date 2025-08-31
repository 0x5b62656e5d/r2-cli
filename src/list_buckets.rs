use aws_sdk_s3::{Client, operation::list_buckets::ListBucketsOutput};

pub async fn list_buckets(client: &Client) -> Result<Vec<String>, anyhow::Error> {
    let res: ListBucketsOutput = client.list_buckets().send().await?;

    Ok(res
        .buckets
        .unwrap()
        .iter()
        .map(|b| b.name.clone().unwrap())
        .collect::<Vec<String>>())
}
