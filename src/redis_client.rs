use redis::{Client, Commands, Connection};
use std::env;

pub async fn get_redis_connection() -> Result<Connection, String> {
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    let client =
        Client::open(redis_url).map_err(|_| "Could not establish Redis connection".to_string())?;
    client
        .get_connection()
        .map_err(|_| "Failed to get Redis connection".to_string())
}

pub async fn save_url(
    connection: &mut Connection,
    short_id: &str,
    url: &str,
    validity: usize,
) -> Result<(), String> {
    connection
        .set_ex(short_id, url, validity as u64)
        .map_err(|_| "Failed to save URL to Redis".to_string())
}

pub async fn fetch_url(connection: &mut Connection, key: &str) -> Result<String, String> {
    connection
        .get(key)
        .map_err(|_| "URL not found or expired".to_string())
}
