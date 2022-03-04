use crate::schema::{banks, bank_card_types, bank_card_brands, bank_cards};
use chrono::{NaiveDateTime, Utc};
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

#[derive(Serialize, AsChangeset, Deserialize, Queryable, Insertable)]
#[table_name = "banks"]
pub struct Bank {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub country: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, AsChangeset, Deserialize, Queryable, Insertable)]
#[table_name = "bank_card_types"]
pub struct BankCardType {
    pub id: Option<i32>,
    pub card_type: Option<String>,
}

#[derive(Serialize, AsChangeset, Deserialize, Queryable, Insertable)]
#[table_name = "bank_card_brands"]
pub struct BankCardCompany {
    pub id: Option<i32>,
    pub name: Option<String>,
}

#[derive(Serialize, AsChangeset, Deserialize, Queryable, Insertable)]
#[table_name = "bank_cards"]
pub struct BankCard {
    pub id: Option<i32>,
    pub bank_id: Option<i32>,
    pub card_type_id: Option<i32>,
    pub card_brand_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Bank {
    pub fn read(conn: &MysqlConnection) -> Result<Vec<Bank>, Error> {
        banks::table.order(banks::name.asc()).load::<Bank>(conn)
    }
}

impl BankCardType {
    pub fn read(conn: &MysqlConnection) -> Result<Vec<BankCardType>, Error> {
        bank_card_types::table.order(bank_card_types::card_type.asc()).load::<BankCardType>(conn)
    }
}

impl BankCardCompany {
    pub fn read(conn: &MysqlConnection) -> Result<Vec<BankCardCompany>, Error> {
        bank_card_brands::table.order(bank_card_brands::name.asc()).load::<BankCardCompany>(conn)
    }
}

impl BankCard {
    pub fn create(model: BankCard, conn: &MysqlConnection) -> Result<BankCard, Error> {
        let new_card = BankCard {
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
            ..model
        };

        let ops = diesel::insert_into(bank_cards::table)
            .values(&new_card)
            .execute(conn);

        match ops {
            Ok(_) => bank_cards::table
                .order(bank_cards::id.desc())
                .first(conn),
            Err(e) => {
                let existing_model = match bank_cards::table
                    .filter(bank_cards::bank_id.eq(&new_card.bank_id))
                    .filter(bank_cards::card_type_id.eq(&new_card.card_type_id))
                    .filter(bank_cards::card_brand_id.eq(&new_card.card_brand_id))
                    .first::<BankCard>(conn) {
                    Ok(u) => u,
                    Err(e) => return Err(e)
                };

                return match existing_model.id {
                    Some(_) => Ok(existing_model),
                    _ => Err(e)
                };
            }
        }
    }
}
