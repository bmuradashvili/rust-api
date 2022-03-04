use crate::schema::{
    users,
    user_cars,
    user_bank_cards,
    car_info,
    bank_card_info,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, Utc};
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;
use crate::jwt;
use crate::jwt::AuthorizationToken;

#[derive(Serialize, AsChangeset, Deserialize, Queryable, Insertable, Associations)]
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

#[derive(Serialize, AsChangeset, Deserialize, Queryable, Insertable)]
#[table_name = "user_cars"]
pub struct UserCar {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub car_model_id: Option<i32>,
    pub year: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, AsChangeset, Deserialize, Queryable, Insertable)]
#[table_name = "user_bank_cards"]
pub struct UserBankCard {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub bank_card_id: Option<i32>,
    pub name_on_card: Option<String>,
    pub expiration: Option<NaiveDate>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[table_name = "car_info"]
#[primary_key(id)]
#[belongs_to(User)]
pub struct CarInfo {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub brand: Option<String>,
    pub model: Option<String>,
    pub transmission_type: Option<String>,
    pub engine_cylinders: Option<i32>,
    pub engine_displacement: Option<BigDecimal>,
    pub year: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[table_name = "bank_card_info"]
#[primary_key(id)]
#[belongs_to(User)]
pub struct BankCardInfo {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub bank: Option<String>,
    pub card_type: Option<String>,
    pub card_company: Option<String>,
    pub name_on_card: Option<String>,
    pub expiration: Option<NaiveDate>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub id: Option<i32>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub cars: Option<Vec<CarInfo>>,
    pub cards: Option<Vec<BankCardInfo>>,
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

    pub fn read_by_id(id: i32, conn: &MysqlConnection) -> Result<UserInfo, Error> {
        let user = match users::table.find(id).first::<User>(conn) {
            Ok(u) => u,
            Err(e) => return Err(e)
        };

        let owned_cars = match car_info::table
            .filter(car_info::user_id.eq(id))
            .load::<CarInfo>(conn) {
            Ok(u) => u,
            Err(e) => return Err(e)
        };

        let owned_cards = match bank_card_info::table
            .filter(bank_card_info::user_id.eq(id))
            .load::<BankCardInfo>(conn) {
            Ok(u) => u,
            Err(e) => return Err(e)
        };

        Ok(UserInfo {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            phone_number: user.phone_number,
            birth_date: user.birth_date,
            cars: Some(owned_cars),
            cards: Some(owned_cards),
        })
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

impl UserCar {
    pub fn create(model: UserCar, conn: &MysqlConnection) -> Result<UserCar, Error> {
        let new_user_car = UserCar {
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
            ..model
        };

        let ops = diesel::insert_into(user_cars::table)
            .values(&new_user_car)
            .execute(conn);

        match ops {
            Ok(_) => user_cars::table
                .order(user_cars::id.desc())
                .first(conn),
            Err(e) => Err(e)
        }
    }
}

impl UserBankCard {
    pub fn create(model: UserBankCard, conn: &MysqlConnection) -> Result<UserBankCard, Error> {
        let new_user_card = UserBankCard {
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
            expiration: Some(NaiveDate::parse_from_str(&model.expiration.unwrap().to_string(), "%Y-%m-%d").unwrap()),
            ..model
        };

        let ops = diesel::insert_into(user_bank_cards::table)
            .values(&new_user_card)
            .execute(conn);

        match ops {
            Ok(_) => user_bank_cards::table
                .order(user_bank_cards::id.desc())
                .first(conn),
            Err(e) => Err(e)
        }
    }
}