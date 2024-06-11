// use axum::{
//     extract::{Json, Path},
//     http::StatusCode,
//     response::{IntoResponse, Redirect},
//     routing::{get, post},
//     Router,

// };

// use nanoid::nanoid;
// use redis::{Client, Commands};
// use serde::{Deserialize, Serialize};
// use std::env;
// use url::Url;

// #[derive(Deserialize)]
// struct ShortenRequest {
//     url: String,
//     validity: u64, // validity in seconds
// }

// #[derive(Serialize)]
// struct ShortenResponse {
//     short_url: String,
// }

// #[tokio::main]
// async fn main() {
//     let app = Router::new()
//         .route("/redirect/:key", get(redirect))
//         .route("/shorten", post(shorten_url));

//     let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
//     println!("Listening : https://localhost:3001")
// }

// async fn shorten_url(Json(payload): Json<ShortenRequest>) -> impl IntoResponse {
//     // Validate the URL
//     if Url::parse(&payload.url).is_err() {
//         return (StatusCode::BAD_REQUEST, "Invalid URL").into_response();
//     }

//     let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
//     let client = Client::open(redis_url).expect("Could not establish Redis connection");
//     let mut connection = client
//         .get_connection()
//         .expect("Failed to get Redis connection");

//     // Generate a unique short ID
//     let short_id = nanoid!(8);

//     // Save the URL to Redis with the specified validity period
//     let _: () = connection
//         .set_ex(&short_id, &payload.url, payload.validity as u64)
//         .expect("Failed to save URL to Redis");

//     let short_url = format!("http://localhost:3000/redirect/{}", short_id);
//     Json(ShortenResponse { short_url }).into_response()
// }

// async fn redirect(Path(key): Path<String>) -> Redirect {
//     println!("Redis key {}", key);

//     let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());

//     let client = Client::open(redis_url).expect("Could not extablish redis connection");

//     let mut connection = client
//         .get_connection()
//         .expect("Failed to retrieve redis connection");

//     match connection.get::<&str, String>(&key) {
//         Ok(url) => Redirect::permanent(&url),
//         Err(_) => Redirect::permanent("/404"),
//     }
// }


//Refactored the above code and split the whole into modules

use axum::{
    routing::{get, post},
    Router,
};

mod handlers;
mod models;
mod redis_client;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/",get(handlers::hello_from_server))
        .route("/redirect/:key", get(handlers::redirect))
        .route("/shorten", post(handlers::shorten_url));


    println!("Server will be started on : http://localhost:3001");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();

 

}
