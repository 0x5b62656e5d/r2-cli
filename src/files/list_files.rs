use crate::util::{build_table, round};
use anyhow::bail;
use aws_sdk_s3::{Client, operation::list_objects_v2::ListObjectsV2Output, types::Object};
use byte_unit::{Byte, UnitType};
use chrono::prelude::*;
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct FileInfo {
    key: String,
    last_modified: String,
    size: String,
}

pub async fn list_files(client: &Client, bucket: &str) -> Result<Table, anyhow::Error> {
    let res: ListObjectsV2Output = client.list_objects_v2().bucket(bucket).send().await?;

    if res.contents.is_none() {
        bail!("No files found in the bucket '{}'", bucket)
    }

    let table: Table = build_table(res.contents.unwrap(), |o: &Object| {
        let size = Byte::from_i64(o.size().unwrap_or_else(|| 0))
            .unwrap()
            .get_appropriate_unit(UnitType::Decimal);

        let timestamp =
            DateTime::from_timestamp_millis(o.last_modified().unwrap().to_millis().unwrap())
                .unwrap()
                .with_timezone(&Local)
                .format("%b %d, %Y - %H:%M:%S")
                .to_string();

        FileInfo {
            key: o.key().unwrap().to_string(),
            last_modified: timestamp,
            size: format!("{:?} {:?}", round(size.get_value(), 2), size.get_unit()),
        }
    });

    Ok(table)
}
