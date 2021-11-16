use serde::{Deserialize, Serialize};

const FOAAS_HOST: &str = "https://foaas.com";

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub message: String,
    pub subtitle: String,
}

pub async fn absolutely(company: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/absolutely/{}/{}", FOAAS_HOST, company, from)).await;
    get_message(response)
}

pub async fn anyway(company: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/anyway/{}/{}", FOAAS_HOST, company, from)).await;
    get_message(response)
}

pub async fn asshole(from: &str) -> ApiResponse {
    let response = request(format!("{}/asshole/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn awesome(from: &str) -> ApiResponse {
    let response = request(format!("{}/awesome/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn back(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/back/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

async fn request(url: String) -> Result<ApiResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;

    let response: reqwest::Response = client
        .get(url)
        .header("Accept", "Application/json")
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => match response.json::<ApiResponse>().await {
            Ok(parsed) => Ok(parsed),
            Err(error) => Err(Box::from(error)),
        },
        other => Err(Box::from(format!("{}", other))),
    }
}

fn get_message(response: Result<ApiResponse, Box<dyn std::error::Error>>) -> ApiResponse {
    match response {
        Ok(data) => data,
        Err(error) => ApiResponse {
            message: format!("Error: {}", error),
            subtitle: String::from("error"),
        },
    }
}
