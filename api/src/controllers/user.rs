use crate::db;
use crate::models::user::User;
use crate::utils::response::*;
use rocket_contrib::json::{Json, JsonError};
use crate::jwt::UserToken;
use crate::utils::error::Error;

#[post("/register", data = "<user>")]
fn register(
    user: Result<Json<User>, JsonError>,
    conn: db::Connection,
) -> Result<ApiResponse, ApiError> {
    match user {
        Ok(u) => {
            let insert = User {
                id: None,
                ..u.into_inner()
            };
            let result = User::create(insert, &conn);
            match result {
                Ok(r) => Ok(success(json!(r))),
                Err(e) => Err(db_error(e)),
            }
        }
        Err(e) => Err(json_error(e)),
    }
}

#[post("/login", data = "<user>")]
fn login(
    user: Result<Json<User>, JsonError>,
    conn: db::Connection,
) -> Result<ApiResponse, ApiError> {
    match user {
        Ok(u) => {
            let result = User::login(u.into_inner(), &conn);
            match result {
                Ok(r) => Ok(success(json!(r))),
                Err(e) => Err(db_error(e)),
            }
        }
        Err(e) => Err(json_error(e)),
    }
}

#[get("/info")]
fn get_details(claims: Result<UserToken, Error>, conn: db::Connection) -> Result<ApiResponse, ApiError> {
    match claims {
        Ok(u) => {
            let result = User::read_by_id(u.user_id, &conn);
            match result {
                Ok(r) => Ok(success(json!(r))),
                Err(e) => Err(db_error(e)),
            }
        }
        Err(e) => Err(unauthorized_error(e)),
    }
}

// -- routes
pub fn routes() -> Vec<rocket::Route> {
    routes![register, login, get_details]
}
