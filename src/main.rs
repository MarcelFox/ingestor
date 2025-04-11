use chrono::{DateTime, Utc};
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::Deserialize;
use serde::Serialize;
use serde_json::{json, Value};
use std::f64;
use std::fmt;

mod services;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UseUnit {
    Cpu,
    Memory,
    Storage,
}

#[derive(Serialize, Deserialize)]
struct PulseData {
    tenant: String,
    product_sku: String,
    used_amount: f64,
    use_unit: UseUnit,
    timestamp: Option<String>,
}

impl fmt::Display for UseUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            UseUnit::Cpu => "cpu",
            UseUnit::Memory => "memory",
            UseUnit::Storage => "storage",
        };
        write!(f, "{}", s)
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

const ONE_HOUR: i64 = 3600;

async fn func(event: LambdaEvent<PulseData>) -> Result<Value, Error> {
    let pulse_data: PulseData = event.payload;

    let pulse_info = format!(
        "{}:{}:{}",
        pulse_data.tenant, pulse_data.product_sku, pulse_data.use_unit
    );
    let usage_key = format!("USAGE:{}", pulse_info);
    let timestamp_key = format!("TIMESTAMP:{}", pulse_info);

    let found_timestamp_key = services::redis::scan_keys(timestamp_key.clone());

    if found_timestamp_key.is_empty() {
        services::redis::set_key_value(timestamp_key, Utc::now().to_rfc3339().to_string());
        services::redis::set_key_value(usage_key.clone(), pulse_data.used_amount.to_string());
    } else {
        let found_timestamp: String = services::redis::get_key_value(found_timestamp_key.clone());
        let timestamp: DateTime<Utc> = DateTime::parse_from_rfc3339(&found_timestamp)
            .unwrap()
            .with_timezone(&Utc);
        services::redis::increment_key(usage_key.clone(), pulse_data.used_amount.to_string());
        let elapsed_time = Utc::now().timestamp() - timestamp.timestamp();

        if elapsed_time > ONE_HOUR {
            println!("Sending Message!");
            services::redis::delete_key(found_timestamp_key);
            return Ok(json!({"message": "Passed 1 hour since last update"}));
        }
    }

    let redis_result = services::redis::get_key_value(usage_key);
    let message = format!(
        "Usage amount from tenant '{}' on SKU '{}' has increased to {}",
        pulse_data.tenant, pulse_data.product_sku, redis_result
    );
    return Ok(json!({"message": message}));
}
