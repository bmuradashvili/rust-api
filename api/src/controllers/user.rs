use rocket::Response;
use rocket::response::status;
use crate::db;
use crate::models::user::User;
use crate::utils::response::*;
use crate::*;
use rocket_contrib::json::{Json, JsonError};
// use crate::jwt::UserToken;

// #[get("/")]
// fn read(conn: db::Connection) -> Result<ApiResponse, ApiError> {
//     let result = User::read(&conn);
//     match result {
//         Ok(r) => Ok(success(json!(r))),
//         Err(e) => Err(db_error(e)),
//     }
// }
//
// #[get("/<id>")]
// fn read_by_id(id: i32, conn: db::Connection) -> Result<ApiResponse, ApiError> {
//     let result = User::read_by_id(id, &conn);
//     match result {
//         Ok(r) => Ok(success(json!(r))),
//         Err(e) => Err(db_error(e)),
//     }
// }

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

// #[post("/login", data = "<user>")]
// fn login(
//     user: Result<Json<User>, JsonError>,
//     conn: db::Connection,
// ) -> Result<ApiResponse, ApiError> {
//     match user {
//         Ok(u) => {
//             let result = User::login(u.into_inner(), &conn);
//             match result {
//                 Ok(r) => Ok(success(json!(r))),
//                 Err(e) => Err(db_error(e)),
//             }
//         }
//         Err(e) => Err(json_error(e)),
//     }
// }

// #[get("/info")]
// fn get_details(id: i32, token: Result<UserToken, status::Custom<Json<Response>>>, conn: db::Connection) -> Result<ApiResponse, ApiError> {
//     let result = User::delete(id, &conn);
//     success(json!(result))
// }

// -- routes
pub fn routes() -> Vec<rocket::Route> {
    routes![register]
}
