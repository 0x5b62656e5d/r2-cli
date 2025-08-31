use crate::client::config::Keys;
use anyhow::Result;
use aws_config::{Region, SdkConfig};
use aws_credential_types::Credentials;
use aws_sdk_s3::{
    Client,
    config::{BehaviorVersion, Builder as S3ConfigBuilder},
};

pub async fn build_client(keys: &Keys, region: String) -> Result<Client> {
    let base: SdkConfig = aws_config::defaults(BehaviorVersion::latest()).load().await;

    let creds: Credentials = Credentials::new(
        keys.key_id.clone(),
        keys.secret_key.clone(),
        None,
        None,
        "custom-env",
    );

    if !keys.endpoint_url.is_empty() {
        let sdk_conf: aws_sdk_s3::Config = S3ConfigBuilder::from(&base)
            .credentials_provider(creds)
            .endpoint_url(keys.endpoint_url.clone())
            .region(Region::new(region))
            .force_path_style(true)
            .behavior_version(BehaviorVersion::latest())
            .build();

        Ok(Client::from_conf(sdk_conf))
    } else {
        let sdk_conf: aws_sdk_s3::Config = S3ConfigBuilder::from(&base)
            .credentials_provider(creds)
            .region(Region::new(region))
            .behavior_version(BehaviorVersion::latest())
            .build();

        Ok(Client::from_conf(sdk_conf))
    }
}
