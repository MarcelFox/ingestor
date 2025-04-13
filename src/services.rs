pub mod redis {
    use redis::Commands;
    use std::env;

    fn get_connection() -> redis::Connection {
        let redis_url: String =
            env::var("REDIS_URL").unwrap_or("redis://localhost:6379/".to_string());
        let client = redis::Client::open(redis_url).unwrap();
        let connection = client.get_connection().unwrap();
        return connection;
    }
    pub fn get_key_value(key: String) -> String {
        let mut conn = get_connection();
        let result: String = conn.get(key).unwrap();
        return result;
    }
    pub fn set_key_value(key: String, value: String) -> String {
        let mut conn = get_connection();
        let _: () = conn.set(&key, value).unwrap();
        return get_key_value(key);
    }
    pub fn delete_key(key: String) {
        let mut conn = get_connection();
        let _: () = conn.del(key).unwrap();
    }
    pub fn increment_key(key: String, value: String) -> String {
        let mut conn = get_connection();
        let _: () = conn.incr(&key, value).unwrap();
        return get_key_value(key);
    }
    pub fn scan_keys(pattern: String) -> String {
        let mut conn = get_connection();
        let keys_found: redis::Iter<String> = match conn.scan_match(format!("{}*", pattern)) {
            Ok(iter) => iter,
            Err(err) => {
                println!("Error scanning keys: {}", err);
                return "".to_string();
            }
        };
        let keys_list = keys_found.collect::<Vec<String>>();
        match keys_list.len() == 1 {
            true => {
                return keys_list[0].clone();
            }
            false => {
                println!("Found keys: {:?}", keys_list);
                return "".to_string();
            }
        };
    }
}

pub mod rabbimq {
    use amqp::{Basic, Session, Table};
    use std::env;

    pub fn send_message(queue_name: &str, message: &str) {
        let rabbit_url: String =
            env::var("RABBITMQ_URL").unwrap_or("amqp://admin:admin@localhost:5672".to_string());
        let mut session = match Session::open_url(&rabbit_url) {
            Ok(session) => session,
            Err(err) => panic!("Cannot create session: {:?}", err),
        };
        let mut channel = session.open_channel(1).ok().expect("Can't open channel");

        channel
            .queue_declare(queue_name, false, true, false, false, false, Table::new())
            .unwrap();

        channel
            .basic_publish(
                "",
                queue_name,
                true,
                false,
                amqp::protocol::basic::BasicProperties {
                    content_type: Some("text".to_string()),
                    ..Default::default()
                },
                message.as_bytes().to_vec(),
            )
            .unwrap();
        channel.close(200, "Bye").unwrap();
        session.close(200, "Good Bye");
    }
}

pub mod http {
    use std::env;

    pub async fn post() -> Result<(), Box<dyn std::error::Error>> {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert("message", "sending");

        let client = reqwest::Client::new();
        let resp = match client
            .post(env::var("PROCESSOR_URL").unwrap_or("http://localhost:9000".to_string()))
            .header("Content-Type", "application/json")
            .json(&map)
            .send()
            .await {
                Ok(resp) => resp,
                Err(err) => panic!("Cannot send request: {:?}", err),
            };
        log::info!("{resp:#?}");
        Ok(())
    }
}
