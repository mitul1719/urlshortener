use redis::{Client, Commands, Connection};
use std::{collections::HashSet, env};

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
    validity: u64,
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

//To be implemented
// pub async fn duplicate_checker(connection: &mut Connection, url: &str) -> bool {
//     // let mut existing_urls: Result<_, redis::RedisError> = connection
//     //     .keys("*");


//     match connection.keys("*") {

//         Ok(a) => print!("{}",a),
        
        
//     }



        
//         // .await
//         // // .map_err(|_| "Failed to retrieve keys from Redis".to_string())?
//         // .filter_map(|key| {
//         //     connection
//         //         .get(&key)
//         //         .ok()
//         //         .and_then(|value| String::from_utf8(value).ok())
//         // })
//         // .collect();

//     // // Check if the URL already exists
//     // if existing_urls.contains(url) {
//     //     return false 
//     // } else {
//     //     return true
//     // }

//     false

// }
