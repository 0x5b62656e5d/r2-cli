use aws_sdk_s3::Client;
use aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Output;
use aws_sdk_s3::types::Object;
use byte_unit::{Byte, UnitType};

pub async fn list_files(client: &Client, bucket: &str) -> Result<Vec<String>, anyhow::Error> {
    let res: ListObjectsV2Output = client.list_objects_v2().bucket(bucket).send().await?;

    Ok(res
        .contents
        .unwrap()
        .iter()
        .map(|o: &Object| {
            let size = Byte::from_i64(o.size().unwrap_or_else(|| 0))
                .unwrap()
                .get_appropriate_unit(UnitType::Decimal)
                .to_string();

            format!(
                "{:?}  {:?}  {:?}",
                o.key().unwrap(),
                o.last_modified().unwrap(),
                size
            )
        })
        .collect::<Vec<String>>())
}
