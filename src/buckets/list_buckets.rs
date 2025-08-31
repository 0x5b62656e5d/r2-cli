use aws_sdk_s3::{Client, operation::list_buckets::ListBucketsOutput};
use tabled::{Table, Tabled, settings::Style};

use crate::util::build_table;

#[derive(Tabled)]
struct Bucket {
    name: String,
}

pub async fn list_buckets(client: &Client) -> Result<Table, anyhow::Error> {
    let res: ListBucketsOutput = client.list_buckets().send().await?;

    let table: Table = build_table(res.buckets.unwrap(), |b: &aws_sdk_s3::types::Bucket| {
        Bucket {
            name: b.name.as_ref().unwrap().to_string(),
        }
    });

    Ok(table)
}
