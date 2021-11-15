use serde::{Deserialize, Serialize};

const FOAAS_HOST: &str = "https://foaas.com";

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub message: String,
    pub subtitle: String,
}

pub async fn absolutely(company: &str, from: &str) -> ApiResponse {
    if let Ok(data) = request(format!("{}/absolutely/{}/{}", FOAAS_HOST, company, from)).await {
        return data;
    } else {
        panic!("Request failed.")
    }
}

pub async fn anyway(company: &str, from: &str) -> ApiResponse {
    if let Ok(data) = request(format!("{}/anyway/{}/{}", FOAAS_HOST, company, from)).await {
        return data;
    } else {
        panic!("Request failed.")
    }
}

pub async fn asshole(from: &str) -> ApiResponse {
    if let Ok(data) = request(format!("{}/asshole/{}", FOAAS_HOST, from)).await {
        return data;
    } else {
        panic!("Request failed.")
    }
}

pub async fn awesome(from: &str) -> ApiResponse {
    if let Ok(data) = request(format!("{}/awesome/{}", FOAAS_HOST, from)).await {
        return data;
    } else {
        panic!("Request failed.")
    }
}

pub async fn back(name: &str, from: &str) -> ApiResponse {
    if let Ok(data) = request(format!("{}/back/{}/{}", FOAAS_HOST, name, from)).await {
        return data;
    } else {
        panic!("Request failed.")
    }
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
        other => panic!("Request failed: {}", other),
    }
}
