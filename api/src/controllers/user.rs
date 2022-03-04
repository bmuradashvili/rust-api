use crate::db;
use crate::models::user::{User, UserBankCard, UserCar};
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

#[post("/car/register", data = "<user_car>")]
fn register_user_car(
    user_car: Result<Json<UserCar>, JsonError>,
    claims: Result<UserToken, Error>,
    conn: db::Connection,
) -> Result<ApiResponse, ApiError> {
    match claims {
        Ok(u) => {
            match user_car {
                Ok(c) => {
                    let insert = UserCar {
                        id: None,
                        user_id: Some(u.user_id),
                        ..c.into_inner()
                    };
                    let result = UserCar::create(insert, &conn);

                    match result {
                        Ok(r) => Ok(success(json!(r))),
                        Err(e) => Err(db_error(e))
                    }
                }
                Err(e) => Err(json_error(e))
            }
        }
        Err(e) => Err(unauthorized_error(e)),
    }
}

#[post("/bank/card/register", data = "<user_bank_card>")]
fn register_user_bank_card(
    user_bank_card: Result<Json<UserBankCard>, JsonError>,
    claims: Result<UserToken, Error>,
    conn: db::Connection,
) -> Result<ApiResponse, ApiError> {
    match claims {
        Ok(u) => {
            match user_bank_card {
                Ok(c) => {
                    let insert = UserBankCard {
                        id: None,
                        user_id: Some(u.user_id),
                        ..c.into_inner()
                    };
                    let result = UserBankCard::create(insert, &conn);

                    match result {
                        Ok(r) => Ok(success(json!(r))),
                        Err(e) => Err(db_error(e))
                    }
                }
                Err(e) => Err(json_error(e))
            }
        }
        Err(e) => Err(unauthorized_error(e)),
    }
}

// -- routes
pub fn routes() -> Vec<rocket::Route> {
    routes![register, login, get_details, register_user_car, register_user_bank_card]
}
