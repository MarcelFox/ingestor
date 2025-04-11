pub mod redis {
    use redis::Commands;

    fn get_connection() -> redis::Connection {
        let client = redis::Client::open("redis://0.0.0.0:6379/").unwrap();
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
    pub fn hello() {
        println!("Hello Redis!")
    }
}
