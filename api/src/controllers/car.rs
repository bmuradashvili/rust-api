use crate::db;
use crate::models::car::{CarBrand, CarTransmissionType};
use crate::utils::response::*;
use rocket_contrib::json::{Json, JsonError};
use crate::jwt::UserToken;
use crate::utils::error::Error;

#[get("/brands")]
fn read_brands(claims: Result<UserToken, Error>, conn: db::Connection) -> Result<ApiResponse, ApiError> {
    match claims {
        Ok(_) => {
            let result = CarBrand::read(&conn);
            match result {
                Ok(r) => Ok(success(json!(r))),
                Err(e) => Err(db_error(e)),
            }
        }
        Err(e) => Err(unauthorized_error(e)),
    }
}

#[get("/transmission-types")]
fn read_transmission_types(claims: Result<UserToken, Error>, conn: db::Connection) -> Result<ApiResponse, ApiError> {
    match claims {
        Ok(_) => {
            let result = CarTransmissionType::read(&conn);
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
    routes![read_brands, read_transmission_types]
}
