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
    pub fn set_key_value(key: String, value: f64) -> String {
        let mut conn = get_connection();
        let _: () = conn.set(&key, value).unwrap();
        return get_key_value(key);
    }
    pub fn hello() {
        println!("Hello Redis!")
    }
}
