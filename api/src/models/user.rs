use crate::schema::users;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{NaiveDate, NaiveDateTime, Utc};
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;
use crate::jwt;
use crate::jwt::AuthorizationToken;

#[derive(Serialize, AsChangeset, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
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
            Ok(_) => users::table
                .order(users::id.desc())
                .first(conn),
            Err(e) => Err(e),
        }
    }

    pub fn read_by_id(id: i32, conn: &MysqlConnection) -> Result<User, Error> {
        users::table.find(id).first(conn)
    }

    pub fn login(user: User, conn: &MysqlConnection) -> Result<AuthorizationToken, Error> {
        let user_to_verify = match users::table
            .filter(users::email.eq(&user.email))
            .first::<User>(conn) {
            Ok(u) => u,
            Err(e) => return Err(e)
        };

        if !user_to_verify.password.as_ref().unwrap().is_empty()
            && verify(&user.password.unwrap(), &user_to_verify.password.unwrap()).unwrap() {
            return Ok(jwt::generate_token(user_to_verify.id.unwrap()));
        }

        return Err(Error::NotFound);
    }
}
