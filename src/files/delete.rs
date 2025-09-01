use anyhow::bail;
use aws_sdk_s3::Client;

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
