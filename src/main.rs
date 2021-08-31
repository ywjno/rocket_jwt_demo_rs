use chrono::{Duration, Local};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::{Deserialize, Serialize};
use rocket::{
    routes,
    serde::json::{serde_json::json, Value},
};

const KEY: &[u8] = b"secret";

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

struct CurrentToken;

// "Bearer token"
impl CurrentToken {
    fn from_request(sub: &str, header_auth: &str) -> Option<CurrentToken> {
        let bearer_token = header_auth.split_whitespace().collect::<Vec<_>>();
        if bearer_token.len() != 2 {
            return None;
        }
        if bearer_token[0] != "Bearer" {
            return None;
        }
        Self::from_jwt(sub, bearer_token[1])
    }

    fn from_jwt(sub: &str, token_string: &str) -> Option<CurrentToken> {
        let validation = Validation {
            sub: Some(sub.to_owned()),
            leeway: 24 * 3600, // 有效期: 1天(24*60*60 秒)
            ..Validation::default()
        };
        match decode::<Claims>(token_string, &DecodingKey::from_secret(KEY), &validation) {
            Ok(token_data) => {
                println!("验证{:?}", token_data.claims);
                return Some(CurrentToken);
            }
            // Err(_) => None,
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
                ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
                _ => panic!("Some other errors"),
            },
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for CurrentToken {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let sub = request.query_value::<&str>("sub").unwrap().unwrap();
        let header_auth = request.headers().get_one("Authorization");
        if let Some(header_auth) = header_auth {
            if let Some(auth) = Self::from_request(sub, header_auth) {
                return Outcome::Success(auth);
            }
        }
        Outcome::Failure((Status::Unauthorized, ()))
    }
}

#[rocket::get("/?<sub>")]
async fn get_jwt(sub: &str) -> Value {
    let my_claims = Claims {
        sub: sub.to_owned(),
        company: "yoshiyama".to_string(),
        exp: (Local::now() + Duration::days(1)).timestamp() as usize,
    };
    let token = match encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(KEY),
    ) {
        Ok(token) => token,
        Err(_) => panic!(),
    };
    json!({ "token": token })
}

#[rocket::get("/value")]
async fn valitation_jwt(_auth: CurrentToken) -> Value {
    json!("get data")
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .mount("/jwt", routes![get_jwt, valitation_jwt])
        .launch()
        .await?;
    Ok(())
}
