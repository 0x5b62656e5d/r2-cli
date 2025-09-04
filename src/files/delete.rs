use anyhow::bail;
use aws_sdk_s3::Client;

/// Deletes a file from an S3 bucket, optionally removing all versions if versioning is enabled
/// # Arguments
/// * `client` - A reference to the S3 client
/// * `bucket` - The name of the bucket
/// * `key` - The key (path) of the file to delete
/// * `has_obj_ver` - A boolean indicating if the S3 service supports object versioning
/// * `force` - A boolean indicating if all versions should be deleted (if versioning is supported)
/// # Returns
/// * `Result<(), anyhow::Error>` - `Ok(())` if successful, error if the operation fails
pub async fn delete_file(
    client: &Client,
    bucket: String,
    key: String,
    has_obj_ver: bool,
    force: bool,
) -> Result<(), anyhow::Error> {
    if force && !has_obj_ver {
        println!(
            "Warning: --force flag has no effect because the active S3 service does not support object versioning."
        );
    }

    if !has_obj_ver || !force {
        client
            .delete_object()
            .bucket(&bucket)
            .key(&key)
            .send()
            .await?;

        return Ok(());
    }

    let r1 = client
        .list_object_versions()
        .bucket(&bucket)
        .prefix(&key)
        .send()
        .await?;

    let versions = r1.versions().len() + r1.delete_markers().len();

    if versions == 0 {
        bail!("No versions found for the specified object");
    }

    println!(
        "{} versions found, deleting all {} versions",
        versions, versions
    );

    for r in r1.versions().iter() {
        client
            .delete_object()
            .bucket(&bucket)
            .key(&key)
            .version_id(r.version_id().unwrap())
            .send()
            .await?;
    }

    for r in r1.delete_markers().iter() {
        client
            .delete_object()
            .bucket(&bucket)
            .key(&key)
            .version_id(r.version_id().unwrap())
            .send()
            .await?;
    }

    Ok(())
}
