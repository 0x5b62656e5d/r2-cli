use aws_sdk_s3::Client;
use aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Output;
use aws_sdk_s3::types::Object;
use byte_unit::{Byte, UnitType};
use tabled::{Table, Tabled, settings::Style};

#[derive(Tabled)]
struct FileInfo {
    key: String,
    last_modified: String,
    size: String,
}

pub async fn list_files(client: &Client, bucket: &str) -> Result<Table, anyhow::Error> {
    let res: ListObjectsV2Output = client.list_objects_v2().bucket(bucket).send().await?;

    let mut table = Table::new(
        res.contents
            .unwrap()
            .iter()
            .map(|o: &Object| {
                let size: String = Byte::from_i64(o.size().unwrap_or_else(|| 0))
                    .unwrap()
                    .get_appropriate_unit(UnitType::Decimal)
                    .to_string();

                FileInfo {
                    key: o.key().unwrap().to_string(),
                    last_modified: o.last_modified().unwrap().to_string(),
                    size: size,
                }
            })
            .collect::<Vec<FileInfo>>(),
    );

    table.with(Style::modern());

    Ok(table)
}
