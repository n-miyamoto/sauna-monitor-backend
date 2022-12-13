use diesel;
use diesel::prelude::*;
//use diesel::post;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::schema::sensor_data;
use crate::schema::sensor_data as sensor_data_schema;


use chrono::NaiveDateTime;
use chrono::{DateTime, Utc};

#[derive(Debug, Queryable)]
pub struct SensorData{
	pub id: u64,
    pub time_stamp: NaiveDateTime,
    pub write_key: String,
    pub data0: Option<f32>,
    pub data1: Option<f32>,
    pub data2: Option<f32>,
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "sensor_data"]
pub struct NewSensorData<'a>{
    pub write_key: &'a str,
    pub data0: Option<f32>,
    pub data1: Option<f32>,
    pub data2: Option<f32>,
}


//impl SensorData{
//	pub fn index(connection: &MysqlConnection) -> Vec<User> {
//		users_schema::dsl::users
//		.load::<User>(connection)
//		.expect("Error loading users")
//	}
//
//	pub fn create(insert_user: NewUser, connection: &MysqlConnection) -> Vec<User> {
//		diesel::insert_into(users_schema::dsl::users)
//			.values(&insert_user)
//			.execute(connection)
//			.expect("Error inserting user");
//
//			users_schema::dsl::users
//			.load::<User>(connection)
//			.expect("Error loading users")
//	}
//}