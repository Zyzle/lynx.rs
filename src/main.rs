use std::collections::HashMap;

use rocket::{serde::{json::Json, Deserialize, Serialize}};

#[macro_use]
extern crate rocket;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct EnvConfig {
    client_id: String,
    client_secret: String,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct GhResponse {
    access_token: String,
    scope: String,
    token_type: String,
}

async fn get_access(code: &str) -> Result<GhResponse, reqwest::Error> {
    let vars = envy::from_env::<EnvConfig>()
        .expect("Lynx needs CLIENT_ID and CLIENT_SECRET environment variables set");

    let mut map = HashMap::new();
    map.insert("client_id", vars.client_id);
    map.insert("client_secret", vars.client_secret);
    map.insert("code", String::from(code));

    let client = reqwest::Client::new();
    client
        .post("https://github.com/login/oauth/access_token")
        .json(&map)
        .header("Accept", "application/json")
        .send()
        .await?
        .json::<GhResponse>()
        .await
}

#[get("/?<code>")]
async fn token(code: String) -> Json<GhResponse> {
    let access_response = get_access(&code).await;

    let access_response = match access_response {
        Ok(resp) => resp,
        Err(_) => GhResponse {
            access_token: "".to_string(),
            scope: "".to_string(),
            token_type: "".to_string(),
        },
    };

    Json(access_response)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/token", routes![token])
}
