use crate::schema::users;
use bcrypt::{hash, verify, DEFAULT_COST};
use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, Utc};
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;
// use crate::jwt;
// use crate::jwt::UserToken;
use crate::schema::users::{email, password};
use crate::utils::response::{db_error, unauthorized_error};

#[table_name = "users"]
#[derive(Serialize, AsChangeset, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: Option<i32>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl User {
    pub fn create(user: User, conn: &MysqlConnection) -> Result<User, Error> {
        let password_hashed = hash(&user.password.unwrap(), DEFAULT_COST).unwrap();
println!("{}", user.birth_date.unwrap().to_string());
        let new_user = User {
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
            password: Some(password_hashed),
            birth_date: Some(NaiveDate::parse_from_str(&user.birth_date.unwrap().to_string(), "%Y-%m-%d").unwrap()),
            ..user
        };

        let ops = diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn);

        match ops {
            Ok(_) => users::table.order(users::id.desc()).first(conn),
            Err(e) => Err(e),
        }
    }

    pub fn read_by_id(id: i32, conn: &MysqlConnection) -> Result<User, Error> {
        users::table.find(id).first(conn)
    }

    // pub fn login(user: User, conn: &MysqlConnection) -> Result<String, Error> {
    //     let user_to_verify = users::table
    //         .filter(users::email.eq(&user.email))
    //         .first(conn)
    //         .unwrap();
    //
    //     if !user_to_verify.password.is_empty()
    //         && verify(&user.password, &user_to_verify.password).unwrap() {
    //         match jwt::generate_token(user_to_verify.id) {
    //             Some(t) => Ok(t),
    //             _ => Ok("fail".to_string())
    //         }
    //     }
    //
    //     match jwt::generate_token(user_to_verify.id) {
    //         Some(t) => Ok(t),
    //         _ => Ok("fail".to_string())
    //     }
    // }

    pub fn delete(id: i32, conn: &MysqlConnection) -> bool {
        diesel::delete(users::table.find(id)).execute(conn).is_ok()
    }
}
