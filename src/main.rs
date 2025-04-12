mod interfaces;
mod services;

use std::env;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use crate::interfaces::PulseData;
use chrono::{DateTime, Utc};
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
    log::info!("Pulse received");
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

const QUEUE_NAME: &str = "pulse_queue";

fn create_new_entry_on_redis(timestamp_key: String, key: String, value: String) {
    services::redis::set_key_value(timestamp_key, Utc::now().to_rfc3339().to_string());
    services::redis::set_key_value(key, value);
}

fn an_hour_has_passed(found_timestamp_key: String) -> bool {
    let redis_entry_ttl: i64 = env::var("REDIS_ENTRY_TTL")
        .unwrap_or("3600".to_string())
        .parse()
        .unwrap();
    let found_timestamp: String = services::redis::get_key_value(found_timestamp_key.clone());
    let timestamp: DateTime<Utc> = DateTime::parse_from_rfc3339(&found_timestamp)
        .unwrap()
        .with_timezone(&Utc);
    let elapsed_time = Utc::now().timestamp() - timestamp.timestamp();
    if elapsed_time > redis_entry_ttl {
        return true;
    }
    return false;
}

fn send_message_to_rabbitmq_and_delete_timestamp_key(key: String, timestamp_key: String) {
    services::rabbimq::send_message(
        QUEUE_NAME,
        format!(
            "{}:{}",
            key.clone(),
            services::redis::get_key_value(key.clone()),
        )
        .as_str(),
    );
    services::redis::delete_key(timestamp_key);
}

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
        create_new_entry_on_redis(
            timestamp_key,
            usage_key.clone(),
            pulse_data.used_amount.to_string(),
        );
    } else {
        services::redis::increment_key(usage_key.clone(), pulse_data.used_amount.to_string());

        if an_hour_has_passed(found_timestamp_key.clone()) {
            send_message_to_rabbitmq_and_delete_timestamp_key(
                usage_key.clone(),
                found_timestamp_key.clone(),
            );
        }
    }

    let redis_result = services::redis::get_key_value(usage_key);
    let message = format!(
        "Usage amount from tenant '{}' on SKU '{}' has increased to {}",
        pulse_data.tenant, pulse_data.product_sku, redis_result
    );
    return Ok(json!({"message": message}));
}
