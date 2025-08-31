use crate::config::Keys;
use anyhow::Result;
use aws_config::SdkConfig;
use aws_credential_types::Credentials;
use aws_sdk_s3::{
    Client,
    config::{BehaviorVersion, Builder as S3ConfigBuilder},
};

pub async fn build_client(keys: &Keys) -> Result<Client> {
    let base: SdkConfig = aws_config::defaults(BehaviorVersion::latest()).load().await;

    let creds: Credentials = Credentials::new(
        keys.key_id.clone(),
        keys.secret_key.clone(),
        None,
        None,
        "custom-env",
    );

    let sdk_conf: aws_sdk_s3::Config = S3ConfigBuilder::from(&base)
        .credentials_provider(creds)
        .endpoint_url(keys.endpoint_url.clone())
        .force_path_style(true)
        .behavior_version(BehaviorVersion::latest())
        .build();

    Ok(Client::from_conf(sdk_conf))
}
