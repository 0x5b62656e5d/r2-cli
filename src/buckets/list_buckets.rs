use aws_sdk_s3::{Client, operation::list_buckets::ListBucketsOutput};
use tabled::{Table, Tabled};

use crate::util::build_table;

#[derive(Tabled)]
/// A simplified representation of an S3 bucket
struct Bucket {
    name: String,
}

/// Lists all the buckets in the S3 account
/// # Arguments
/// * `client` - A reference to the S3 client
/// # Returns
/// * `Result<Table, anyhow::Error>` - `Table` if successful, error if the operation fails
pub async fn list_buckets(client: &Client) -> Result<Table, anyhow::Error> {
    let res: ListBucketsOutput = client.list_buckets().send().await?;

    let table: Table = build_table(res.buckets.unwrap(), |b: &aws_sdk_s3::types::Bucket| {
        Bucket {
            name: b.name.as_ref().unwrap().to_string(),
        }
    });

    Ok(table)
}
