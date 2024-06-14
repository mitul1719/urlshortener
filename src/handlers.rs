use crate::models::{ErrorResponse, ShortenRequest, ShortenResponse};
use crate::redis_client::get_redis_connection;
use crate::Database;
use axum::response::Html;
use axum::Extension;
use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use nanoid::nanoid;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::info;
use url::Url;

pub async fn shorten_url(
    Json(payload): Json<ShortenRequest>,
    Extension(database): Extension<Arc<Mutex<Database>>>,
) -> impl IntoResponse {
    // Validating the URL
    if Url::parse(&payload.url).is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid URL".into(),
            }),
        )
            .into_response();
    }

    let mut connection = match get_redis_connection().await {
        Ok(conn) => conn,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse { error: err }),
            )
                .into_response();
        }
    };

    //TODO : Needs to be implemented duplicate checking
    // if duplicate_checker(&mut connection, &payload.url).await {
    //     return (
    //         StatusCode::CONFLICT,
    //         Json(ErrorResponse {
    //             error: "URL already exists".into(),
    //         }),
    //     )
    //         .into_response();
    // }

    // Generate a unique short ID
    let short_id: String = nanoid!(8);

    // Insert key-value pairs with expiration durations
    database.lock().unwrap().insert(
        short_id.to_string(),
        payload.url.to_string(),
        Duration::from_secs(payload.validity),
    );

    //We Should provide deployed url here
    let short_url = format!("http://localhost:3001/redirect/{}", short_id);
    Json(ShortenResponse {
        short_url,
        validity: payload.validity,
    })
    .into_response()

    // // Save the URL to Redis with the specified validity period
    // if let Err(err) = save_url(
    //     &mut connection,
    //     &short_id,
    //     &payload.url,
    //     payload.validity as u64,
    // )
    // .await
    // {
    //     return (
    //         StatusCode::INTERNAL_SERVER_ERROR,
    //         Json(ErrorResponse { error: err }),
    //     )
    //         .into_response();
    // }
    // //We Should provide deployed url here
    // let short_url = format!("http://localhost:3001/redirect/{}", short_id);
    // Json(ShortenResponse {
    //     short_url,
    //     validity: payload.validity,
    // })
    // .into_response()
}

pub async fn redirect(
    Path(key): Path<String>,
    Extension(database): Extension<Arc<Mutex<Database>>>,
) -> impl IntoResponse {
    // let mut connection = match get_redis_connection().await {
    //     Ok(conn) => conn,
    //     Err(err) => {
    //         return (
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             Json(ErrorResponse { error: err }),
    //         )
    //             .into_response();
    //     }
    // };

    match database.lock().unwrap().get(&key) {
        Some(kv) => {
            info!("Value of kv.value: {}", kv.value);
            Redirect::permanent(&kv.value).into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "URL not found".into(),
            }),
        )
            .into_response(),
    }

    // match fetch_url(&mut connection, &key).await {
    //     Ok(url) => Redirect::permanent(&url).into_response(),
    //     Err(err) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: err })).into_response(),
    // }
}

pub async fn hello_from_server() -> impl IntoResponse {
    return Html("<h1>Hello from url shortening service</h1>");
}
