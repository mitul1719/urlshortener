use crate::models::{ErrorResponse, ShortenRequest, ShortenResponse};
use crate::redis_client::{ fetch_url, get_redis_connection, save_url};
use axum::response::Html;
use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use nanoid::nanoid;
use url::Url;

pub async fn shorten_url(Json(payload): Json<ShortenRequest>) -> impl IntoResponse {
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
    let short_id = nanoid!(8);

    // Save the URL to Redis with the specified validity period
    if let Err(err) = save_url(
        &mut connection,
        &short_id,
        &payload.url,
        payload.validity as u64,
    )
    .await
    {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: err }),
        )
            .into_response();
    }
    //We Should provide deployed url here
    let short_url = format!("http://localhost:3001/redirect/{}", short_id);
    Json(ShortenResponse {
        short_url,
        validity: payload.validity,
    })
    .into_response()
}

pub async fn redirect(Path(key): Path<String>) -> impl IntoResponse {
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

    match fetch_url(&mut connection, &key).await {
        Ok(url) => Redirect::permanent(&url).into_response(),
        Err(err) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: err })).into_response(),
    }
}

pub async fn hello_from_server() -> impl IntoResponse {
    return Html("<h1>Hello from url shortening service</h1>");
}
