use serde::{Deserialize, Serialize};

const FOAAS_HOST: &str = "https://foaas.com";

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub message: String,
    pub subtitle: String,
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

//===========================================================================
// Response handlers
// Each function here represents a foaas route
// @TODO Move these outside of this file and find a way to organize them
//===========================================================================
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

pub async fn bag(from: &str) -> ApiResponse {
    let response = request(format!("{}/bag/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn ballmer(name: &str, company: &str, from: &str) -> ApiResponse {
    let response = request(format!(
        "{}/ballmer/{}/{}/{}",
        FOAAS_HOST, name, company, from
    ))
    .await;
    get_message(response)
}

pub async fn bday(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/bday/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn because(from: &str) -> ApiResponse {
    let response = request(format!("{}/because/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn blackadder(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/blackadder/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn bm(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/bm/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn bucket(from: &str) -> ApiResponse {
    let response = request(format!("{}/bucket/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn bus(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/bus/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn bye(from: &str) -> ApiResponse {
    let response = request(format!("{}/bye/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn caniuse(tool: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/caniuse/{}/{}", FOAAS_HOST, tool, from)).await;
    get_message(response)
}

pub async fn chainsaw(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/chainsaw/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn cocksplat(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/cocksplat/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn cool(from: &str) -> ApiResponse {
    let response = request(format!("{}/cool/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn cup(from: &str) -> ApiResponse {
    let response = request(format!("{}/cup/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn dalton(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/dalton/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn dense(from: &str) -> ApiResponse {
    let response = request(format!("{}/dense/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn deraadt(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/deraadt/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn diabetes(from: &str) -> ApiResponse {
    let response = request(format!("{}/diabetes/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn donut(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/donut/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn do_something(to_do: &str, something: &str, from: &str) -> ApiResponse {
    let response = request(format!(
        "{}/dosomething/{}/{}/{}",
        FOAAS_HOST, to_do, something, from
    ))
    .await;
    get_message(response)
}

pub async fn dumbledore(from: &str) -> ApiResponse {
    let response = request(format!("{}/dumbledore/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn equity(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/equity/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn even(from: &str) -> ApiResponse {
    let response = request(format!("{}/even/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn everyone(from: &str) -> ApiResponse {
    let response = request(format!("{}/everyone/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn everything(from: &str) -> ApiResponse {
    let response = request(format!("{}/everything/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn family(from: &str) -> ApiResponse {
    let response = request(format!("{}/family/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn fascinating(from: &str) -> ApiResponse {
    let response = request(format!("{}/fascinating/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn fewer(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/fewer/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn field(name: &str, from: &str, reference: &str) -> ApiResponse {
    let response = request(format!(
        "{}/field/{}/{}/{}",
        FOAAS_HOST, name, from, reference
    ))
    .await;
    get_message(response)
}

pub async fn flying(from: &str) -> ApiResponse {
    let response = request(format!("{}/flying/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn ftfy(from: &str) -> ApiResponse {
    let response = request(format!("{}/ftfy/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn fts(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/fts/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn fyyff(from: &str) -> ApiResponse {
    let response = request(format!("{}/fyyff/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn gfy(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/gfy/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn give(from: &str) -> ApiResponse {
    let response = request(format!("{}/give/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn greed(noun: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/greed/{}/{}", FOAAS_HOST, noun, from)).await;
    get_message(response)
}

pub async fn holy_grail(from: &str) -> ApiResponse {
    let response = request(format!("{}/holygrail/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn horse(from: &str) -> ApiResponse {
    let response = request(format!("{}/horse/{}", FOAAS_HOST, from)).await;
    get_message(response)
}
