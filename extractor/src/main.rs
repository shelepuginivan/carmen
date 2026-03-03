use extractor::config::Config;
use extractor::documents::DocumentAdapter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env("CARMEN_EXTRACTOR_LOG");

    let config = Config::read_from_env()?;

    //let region = Region::Custom {
    //    region: config.s3_region,
    //    endpoint: config.s3_endpoint,
    //};

    //let credentials = Credentials::new(
    //    Some(&config.s3_access_key),
    //    Some(&config.s3_secret_key),
    //    None,
    //    None,
    //    None,
    //)?;

    //let bucket = Bucket::new(&config.s3_bucket, region, credentials)?.with_path_style();

    //let s3_path = "test.file";
    //let test = b"I'm going to S3!";

    //let response_data = bucket.put_object(s3_path, test).await?;
    //assert_eq!(response_data.status_code(), 200);

    let document_adapter = DocumentAdapter::new(&config)?;

    document_adapter.handle().await;

    Ok(())
}
