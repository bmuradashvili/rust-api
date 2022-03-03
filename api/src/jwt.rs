use std::env;
use chrono::Utc;
use dotenv::dotenv;
use jsonwebtoken::TokenData;
use jsonwebtoken::{Header, EncodingKey, DecodingKey, Validation};
use jsonwebtoken::errors::{Error as JWTError, ErrorKind};
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use crate::utils::error::Error;

static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizationToken {
    pub access_token: String,
    pub claims: UserToken,
}

impl<'a, 'r> FromRequest<'a, 'r> for UserToken {
    type Error = Error;

    fn from_request(
        request: &'a Request<'r>,
    ) -> request::Outcome<Self, Self::Error> {
        match request.headers().get_one("Authorization") {
            Some(header_value) => {
                let auth_str = header_value.to_string();
                if header_value.starts_with("Bearer") {
                    let token = auth_str[6..auth_str.len()].trim();
                    return match decode_token(token) {
                        Ok(t) => Outcome::Success(t.claims),
                        Err(e) => Outcome::Failure((
                            Status::Unauthorized,
                            match *e.kind() {
                                ErrorKind::InvalidToken => Error::AuthorizationTokenInvalid,
                                ErrorKind::ExpiredSignature => Error::AuthorizationTokenExpired,
                                _ => Error::AuthorizationHeaderMissing,
                            })
                        )
                    };
                }
                Outcome::Failure((Status::Unauthorized, Error::AuthorizationTokenInvalid))
            }
            None => Outcome::Failure((Status::Unauthorized, Error::AuthorizationHeaderMissing))
        }
    }
}

pub fn generate_token(user_id: i32) -> AuthorizationToken {
    dotenv().ok();
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
    let payload = UserToken {
        iat: now,
        exp: now + ONE_WEEK,
        user_id,
    };

    let access_token = jsonwebtoken::encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(secret_key.as_ref()),
    ).unwrap();

    return AuthorizationToken {
        access_token,
        claims: payload,
    };
}

fn decode_token(token: &str) -> Result<TokenData<UserToken>, JWTError> {
    dotenv().ok();
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    match jsonwebtoken::decode::<UserToken>(&token, &DecodingKey::from_secret(secret_key.as_ref()), &Validation::default()) {
        Ok(t) => Ok(t),
        Err(e) => Err(e)
    }
}