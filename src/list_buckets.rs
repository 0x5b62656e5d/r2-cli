use aws_sdk_s3::{Client, operation::list_buckets::ListBucketsOutput};
use tabled::{Table, Tabled, settings::Style};

#[derive(Tabled)]
struct Bucket {
    name: String,
}

pub async fn list_buckets(client: &Client) -> Result<Table, anyhow::Error> {
    let res: ListBucketsOutput = client.list_buckets().send().await?;

    let mut table = Table::new(
        res.buckets
            .unwrap()
            .iter()
            .map(|b| Bucket {
                name: b.name.as_ref().unwrap().to_string(),
            })
            .collect::<Vec<Bucket>>(),
    );

    table.with(Style::modern());

    Ok(table)
}
