use aws_config::{BehaviorVersion, SdkConfig};
use aws_sdk_s3 as s3;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    command: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

// Return the Client connection with S3
async fn get_s3_client() -> s3::Client {
    let config: SdkConfig = aws_config::defaults(BehaviorVersion::latest()).load().await;
    s3::Client::new(&config)
}

// Return bucket list as vector strings
async fn get_bucket_list(s3_client: &s3::Client) -> Vec<String> {
    let buckets = s3_client.list_buckets().send().await.unwrap();
    let bucket_list: Vec<String> = buckets
        .buckets()
        .iter()
        .map(|b| b.name().unwrap().to_string())
        .collect();
    bucket_list
}

// Function handler
async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let _command = event.payload.command;
    // Get bucket s3 list
    let s3_client: s3::Client = get_s3_client().await;
    let bucket_list: Vec<String> = get_bucket_list(&s3_client).await;

    // Prepare the response with the buckets list
    let resp: Response = Response {
        req_id: event.context.request_id,
        msg: format!("Bucket list: {:?} ", bucket_list),
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
