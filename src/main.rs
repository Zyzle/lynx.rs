use std::collections::HashMap;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::response::status::NotFound;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{Request, Response};

#[macro_use]
extern crate rocket;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct EnvConfig {
    client_id: String,
    client_secret: String,
    github_base: String,
}

#[derive(Deserialize, Debug, Serialize, PartialEq)]
#[serde(crate = "rocket::serde")]
struct GhResponse {
    access_token: String,
    scope: String,
    token_type: String,
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

async fn get_access(code: &str) -> Result<GhResponse, reqwest::Error> {
    let vars = envy::from_env::<EnvConfig>()
        .expect("Lynx needs CLIENT_ID, CLIENT_SECRET, and GITHUB_BASE environment variables set");

    let mut map = HashMap::new();
    map.insert("client_id", vars.client_id);
    map.insert("client_secret", vars.client_secret);
    map.insert("code", String::from(code));

    let url = vars.github_base;

    let client = reqwest::Client::new();
    client
        .post(url)
        .json(&map)
        .header("Accept", "application/json")
        .send()
        .await?
        .json::<GhResponse>()
        .await
}

#[get("/?<code>")]
async fn token(code: String) -> Result<Json<GhResponse>, NotFound<String>> {
    let access_response = get_access(&code).await;

    match access_response {
        Ok(resp) => (Ok(Json(resp))),
        Err(_) => Err(NotFound("Not Found".to_string())),
    }
}

#[launch]
fn rocket() -> _ {
    //  Want to panic immediately if env vars aren't set
    let _vars = envy::from_env::<EnvConfig>()
        .expect("Lynx needs CLIENT_ID, CLIENT_SECRET, and GITHUB_BASE environment variables set");
    rocket::build().mount("/token", routes![token]).attach(CORS)
}

#[cfg(test)]
mod tests {
    use super::rocket;
    use httpmock::prelude::*;
    use rocket::serde::json::serde_json::json;

    use super::*;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_get_access_ok() -> Result<(), reqwest::Error> {
        let server = MockServer::start();
        std::env::set_var("GITHUB_BASE", server.url("/"));
        std::env::set_var("CLIENT_ID", "123");
        std::env::set_var("CLIENT_SECRET", "ABC");

        let mock_access = server.mock(|when, then| {
            when.method(POST).path("/");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({
                    "access_token": "5678",
                    "scope": "",
                    "token_type": "Bearer"
                }));
        });

        let response = aw!(get_access("1234"))?;
        let expected = GhResponse {
            access_token: "5678".to_string(),
            scope: "".to_string(),
            token_type: "Bearer".to_string(),
        };

        mock_access.assert();

        assert_eq!(expected, response);

        Ok(())
    }

    #[test]
    #[should_panic(
        expected = "Lynx needs CLIENT_ID, CLIENT_SECRET, and GITHUB_BASE environment variables set"
    )]
    fn test_get_access_env_panic() {
        std::env::remove_var("GITHUB_BASE");

        let _result = aw!(get_access("1234"));
    }
}
