use bigdecimal::BigDecimal;
use crate::schema::{car_brands, car_models, car_transmission_types};
use chrono::{NaiveDateTime, Utc};
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

#[derive(Serialize, AsChangeset, Deserialize, Queryable, Insertable)]
#[table_name = "car_brands"]
pub struct CarBrand {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, AsChangeset, Deserialize, Queryable, Insertable)]
#[table_name = "car_transmission_types"]
pub struct CarTransmissionType {
    pub id: Option<i32>,
    pub name: Option<String>,
}

#[derive(Serialize, AsChangeset, Deserialize, Queryable, Insertable)]
#[table_name = "car_models"]
pub struct CarModel {
    pub id: Option<i32>,
    pub brand_id: Option<i32>,
    pub transmission_type_id: Option<i32>,
    pub model_name: Option<String>,
    pub engine_cylinders: Option<i32>,
    pub engine_displacement: Option<BigDecimal>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl CarBrand {
    pub fn read(conn: &MysqlConnection) -> Result<Vec<CarBrand>, Error> {
        car_brands::table.order(car_brands::name.asc()).load::<CarBrand>(conn)
    }
}

impl CarTransmissionType {
    pub fn read(conn: &MysqlConnection) -> Result<Vec<CarTransmissionType>, Error> {
        car_transmission_types::table.order(car_transmission_types::name.asc()).load::<CarTransmissionType>(conn)
    }
}

impl CarModel {
    pub fn create(model: CarModel, conn: &MysqlConnection) -> Result<CarModel, Error> {
        let new_model = CarModel {
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
            ..model
        };

        let ops = diesel::insert_into(car_models::table)
            .values(&new_model)
            .execute(conn);

        match ops {
            Ok(_) => car_models::table
                .order(car_models::id.desc())
                .first(conn),
            Err(e) => {
                let existing_model = match car_models::table
                    .filter(car_models::brand_id.eq(&new_model.brand_id))
                    .filter(car_models::transmission_type_id.eq(&new_model.transmission_type_id))
                    .filter(car_models::model_name.eq(&new_model.model_name))
                    .filter(car_models::engine_cylinders.eq(&new_model.engine_cylinders))
                    .filter(car_models::engine_displacement.eq(&new_model.engine_displacement))
                    .first::<CarModel>(conn) {
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
