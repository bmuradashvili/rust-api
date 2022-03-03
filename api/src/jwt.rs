// use std::env;
// use chrono::Utc;
// use dotenv::dotenv;
// use jsonwebtoken::TokenData;
// use jsonwebtoken::{Header, Validation};
// use jsonwebtoken::{EncodingKey, DecodingKey};
// use jsonwebtoken::errors::{Error as JWTError, ErrorKind};
// use rocket::http::Status;
// use rocket::outcome::Outcome;
// use rocket::request::{self, FromRequest, Request};
// use rocket::response::status;
// use rocket_contrib::json::Json;
// use crate::models::user::User;
// use crate::utils::response::unauthorized_error;
//
// static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct UserToken {
//     // issued at
//     pub iat: i64,
//     // expiration
//     pub exp: i64,
//     // data
//     pub user_id: i32,
// }
//
// impl<'a, 'r> FromRequest<'a, 'r> for UserToken {
//     type Error = ();
//
//     fn from_request(
//         request: &'a Request<'r>,
//     ) -> request::Outcome<Self, ()> {
//         if let Some(auth_header) = request.headers().get_one("Authorization") {
//             let auth_str = auth_header.to_string();
//             if auth_str.starts_with("Bearer") {
//                 dotenv().ok();
//                 let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
//
//                 let token = auth_str[6..auth_str.len()].trim();
//                 match decode_token(token.to_string(), &secret_key) {
//                     Ok(t) => Outcome::Success(t.claims),
//                     Err(_) => Outcome::Failure((Status::Unauthorized, ()))
//                 }
//             }
//         }
//         return Outcome::Failure((Status::Unauthorized, ()));
//     }
// }
//
// pub fn generate_token(user_id: i32) -> String {
//     dotenv().ok();
//     let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
//
//     let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
//     let payload = UserToken {
//         iat: now,
//         exp: now + ONE_WEEK,
//         user_id,
//     };
//
//     jsonwebtoken::encode(&Header::default(), &payload, &EncodingKey::from_secret(secret_key.as_ref())).unwrap()
// }
//
// fn decode_token(token: String, secret_key: &String) -> Result<TokenData<UserToken>, JWTError> {
//     match jsonwebtoken::decode::<UserToken>(&token, &DecodingKey::from_secret(secret_key.as_ref()), &Validation::default()) {
//         Ok(t) => Ok(t),
//         Err(e) => Err(e)
//     }
// }