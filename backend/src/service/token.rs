use std::env;

use chrono::Utc;
use common::user::Model as User;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, errors::Error};
use serde::{Deserialize, Serialize};

static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // seconds

// This is to be used within the API
#[derive(Serialize, Deserialize)]
pub struct UserToken {
    pub iat: i64, // issued at (posix)
    pub exp: i64, // expires at (posix)
    pub username: String,
    pub uuid: String, // user id
}

impl UserToken {
    pub fn decode_from_string(token: String) -> Result<Self, Error> {
        let secret = env::var("SECRET").expect("SECRET should be set .env");
        let token_data = jsonwebtoken::decode::<UserToken>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )?;
        Ok(token_data.claims)
    }

    pub fn is_still_valid(&self) -> bool {
        let now = Utc::now().timestamp_millis() / 1000; //seconds
        now < self.exp
    }
}


pub fn generate_token_response(user: &User) -> Result<String, Error> {
    let secret = env::var("SECRET").expect("SECRET should be set .env");
    let now = Utc::now().timestamp_millis() / 1000; //seconds
    let payload = UserToken {
        iat: now,
        exp: now + ONE_WEEK,
        username: user.username.to_string(),
        uuid: user.uuid.to_string(),
    };
    jsonwebtoken::encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}
