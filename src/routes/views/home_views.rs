use actix_web::{get, HttpResponse};
use serde_json::json;
use reqwest::Error as ReqwestError;
use serde::{Deserialize, Serialize};
use log::{error, info};

#[derive(Deserialize)]
struct DogAPIResponse {
    message: String,
    status: String,
}

#[derive(Serialize)]
struct ApiResponse {
    message: String,
    status: String,
    source: String,
}

#[get("")]
pub async fn index() -> HttpResponse {
    // Fetch the dog image data, propagate errors with the `?` operator
    let response = fetch_dog_image().await;

    match response {
        Ok(dog) => {
            info!("Successfully fetched dog image: {}", dog.message);
            HttpResponse::Ok().json(ApiResponse {
                message: dog.message,
                status: dog.status,
                source: "Dog API".to_string(),
            })
        }
        Err(e) => {
            error!("Error fetching dog image: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to fetch data from Dog API",
                "details": e.to_string(),
            }))
        }
    }
}

async fn fetch_dog_image() -> Result<DogAPIResponse, ReqwestError> {
    reqwest::get("https://dog.ceo/api/breeds/image/random")
        .await?
        .json()
        .await
}
