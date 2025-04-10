use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde::Serialize;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize, Serialize)]
struct EventData {
    tenant: String,
    product_sku: String,
    used_amount: f32,
    use_unit: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    let first_name = event["firstName"].as_str().unwrap_or("world");

    Ok(json!({ "message": format!("Hello, {}!", first_name) }))
}
