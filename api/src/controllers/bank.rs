use crate::db;
use crate::models::bank::{Bank, BankCard, BankCardCompany, BankCardType};
use crate::utils::response::*;
use rocket_contrib::json::{Json, JsonError};
use crate::jwt::UserToken;
use crate::utils::error::Error;

#[get("/companies")]
fn read_bank_companies(claims: Result<UserToken, Error>, conn: db::Connection) -> Result<ApiResponse, ApiError> {
    match claims {
        Ok(_) => {
            let result = Bank::read(&conn);
            match result {
                Ok(r) => Ok(success(json!(r))),
                Err(e) => Err(db_error(e)),
            }
        }
        Err(e) => Err(unauthorized_error(e)),
    }
}

#[get("/card/types")]
fn read_card_types(claims: Result<UserToken, Error>, conn: db::Connection) -> Result<ApiResponse, ApiError> {
    match claims {
        Ok(_) => {
            let result = BankCardType::read(&conn);
            match result {
                Ok(r) => Ok(success(json!(r))),
                Err(e) => Err(db_error(e)),
            }
        }
        Err(e) => Err(unauthorized_error(e)),
    }
}

#[get("/card/companies")]
fn read_card_companies(claims: Result<UserToken, Error>, conn: db::Connection) -> Result<ApiResponse, ApiError> {
    match claims {
        Ok(_) => {
            let result = BankCardCompany::read(&conn);
            match result {
                Ok(r) => Ok(success(json!(r))),
                Err(e) => Err(db_error(e)),
            }
        }
        Err(e) => Err(unauthorized_error(e)),
    }
}

#[post("/card/register", data = "<bank_card>")]
fn register_bank_card(
    bank_card: Result<Json<BankCard>, JsonError>,
    claims: Result<UserToken, Error>,
    conn: db::Connection,
) -> Result<ApiResponse, ApiError> {
    match claims {
        Ok(_) => {
            match bank_card {
                Ok(c) => {
                    let insert = BankCard {
                        id: None,
                        ..c.into_inner()
                    };
                    let result = BankCard::create(insert, &conn);

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
    routes![read_bank_companies, read_card_types, read_card_companies, register_bank_card]
}
