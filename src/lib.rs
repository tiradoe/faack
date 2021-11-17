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

pub async fn idea(from: &str) -> ApiResponse {
    let response = request(format!("{}/idea/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn immensity(from: &str) -> ApiResponse {
    let response = request(format!("{}/immensity/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn ing(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/ing/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn jingle_bells(from: &str) -> ApiResponse {
    let response = request(format!("{}/jinglebells/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn keep(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/keep/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn keep_calm(reaction: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/keepcalm/{}/{}", FOAAS_HOST, reaction, from)).await;
    get_message(response)
}

pub async fn king(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/king/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn legend(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/legend/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn life(from: &str) -> ApiResponse {
    let response = request(format!("{}/life/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn linus(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/linus/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn logs(from: &str) -> ApiResponse {
    let response = request(format!("{}/logs/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn look(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/look/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn looking(from: &str) -> ApiResponse {
    let response = request(format!("{}/looking/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn low_poly(from: &str) -> ApiResponse {
    let response = request(format!("{}/lowpoly/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn madison(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/madison/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn maybe(from: &str) -> ApiResponse {
    let response = request(format!("{}/maybe/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn me(from: &str) -> ApiResponse {
    let response = request(format!("{}/me/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn mornin(from: &str) -> ApiResponse {
    let response = request(format!("{}/mornin/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn no(from: &str) -> ApiResponse {
    let response = request(format!("{}/no/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn nugget(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/nugget/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn off(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/off/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn off_with(behavior: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/off-with/{}/{}", FOAAS_HOST, behavior, from)).await;
    get_message(response)
}

pub async fn outside(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/outside/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn particular(thing: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/particular/{}/{}", FOAAS_HOST, thing, from)).await;
    get_message(response)
}

pub async fn pink(from: &str) -> ApiResponse {
    let response = request(format!("{}/pink/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn problem(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/problem/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn programmer(from: &str) -> ApiResponse {
    let response = request(format!("{}/programmer/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn pulp(language: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/pulp/{}/{}", FOAAS_HOST, language, from)).await;
    get_message(response)
}

pub async fn question(from: &str) -> ApiResponse {
    let response = request(format!("{}/question/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn rats_arse(from: &str) -> ApiResponse {
    let response = request(format!("{}/ratsarse/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn retard(from: &str) -> ApiResponse {
    let response = request(format!("{}/retard/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn ridiculous(from: &str) -> ApiResponse {
    let response = request(format!("{}/ridiculous/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn rockstar(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/rockstar/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn rtfm(from: &str) -> ApiResponse {
    let response = request(format!("{}/rtfm/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn sake(from: &str) -> ApiResponse {
    let response = request(format!("{}/sake/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn shakespeare(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/shakespeare/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn shit(from: &str) -> ApiResponse {
    let response = request(format!("{}/shit/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn shutup(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/shutup/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn single(from: &str) -> ApiResponse {
    let response = request(format!("{}/single/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn thanks(from: &str) -> ApiResponse {
    let response = request(format!("{}/thanks/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn that(from: &str) -> ApiResponse {
    let response = request(format!("{}/that/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn think(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/think/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn thinking(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/thinking/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn this(from: &str) -> ApiResponse {
    let response = request(format!("{}/this/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn thumbs(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/thumbs/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn too(from: &str) -> ApiResponse {
    let response = request(format!("{}/too/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn tucker(from: &str) -> ApiResponse {
    let response = request(format!("{}/tucker/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn understand(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/understand/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn waste(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/waste/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn what(from: &str) -> ApiResponse {
    let response = request(format!("{}/what/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn xmas(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/xmas/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn yeah(from: &str) -> ApiResponse {
    let response = request(format!("{}/yeah/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn yoda(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/yoda/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn you(name: &str, from: &str) -> ApiResponse {
    let response = request(format!("{}/you/{}/{}", FOAAS_HOST, name, from)).await;
    get_message(response)
}

pub async fn zayn(from: &str) -> ApiResponse {
    let response = request(format!("{}/zayn/{}", FOAAS_HOST, from)).await;
    get_message(response)
}

pub async fn zero(from: &str) -> ApiResponse {
    let response = request(format!("{}/zero/{}", FOAAS_HOST, from)).await;
    get_message(response)
}
