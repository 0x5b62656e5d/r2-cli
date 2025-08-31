# s3-cli

A simple CLI tool to manage S3 buckets and files from the terminal

## Setup

This tool requires API keys acquired from a S3 service. API keys must be granted `Admin Read & Write` in order for bucket and file management to work properly.

### Environment variables setup

Create a `config.toml` file under `~/.config/s3-cli/`

Configuration template:
```toml
[default]
key_id = "Key ID"
secret_key = "Secret Key"
endpoint_url = "Endpoint URL"
```

## Usage
```
NAME
    s3-cli - Manages S3 buckets and files

COMMANDS
    buckets list
        Lists buckets associated with the S3 account
    buckets create <bucket_name>
        Creates a bucket
    buckets delete <bucket_name>
        Deletes a bucket

    files list <bucket_name>
        Lists all the files in a bucket
    files delete <bucket_name> <file_name>
        Deletes a file in a bucket
    files download <bucket_name> <file_key> <download_location> [-o <override_downloaded_filename>]
        Downloads a file from a bucket to a given location (optionally rename it)
    files upload <bucket_name> <file_location> [-o <override_uploaded_filename>]
        Uploads a file into a bucket (optionally rename it)
```