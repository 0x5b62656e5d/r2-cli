use aws_sdk_s3::{
    Client,
    types::{BucketLocationConstraint, CreateBucketConfiguration},
};

pub async fn create_bucket(
    client: &Client,
    name: String,
    region: String,
) -> Result<(), anyhow::Error> {
    client
        .create_bucket()
        .bucket(name)
        .create_bucket_configuration(
            CreateBucketConfiguration::builder()
                .location_constraint(BucketLocationConstraint::from(region.as_str()))
                .build(),
        )
        .send()
        .await?;
    Ok(())
}
