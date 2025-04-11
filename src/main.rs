use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::Deserialize;
use serde::Serialize;
use serde_json::{json, Value};
use std::f64;

mod services;

#[derive(Serialize, Deserialize)]
struct PulseData {
    tenant: String,
    product_sku: String,
    used_amount: f64,
    use_unit: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: LambdaEvent<PulseData>) -> Result<Value, Error> {
    services::redis::hello();
    let pulse_data = event.payload;
    let key_name = format!(
        "USAGE:{}:{}:{}",
        pulse_data.tenant, pulse_data.product_sku, pulse_data.use_unit
    );
    let redis_result = services::redis::set_key_value(key_name, pulse_data.used_amount);
    let message = format!(
        "Usage amount from tenant '{}' on SKU '{}' has increased to {}",
        pulse_data.tenant, pulse_data.product_sku, redis_result
    );
    return Ok(json!({"message": message}));
}
