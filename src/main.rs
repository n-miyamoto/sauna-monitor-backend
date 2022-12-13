#![feature(decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::response::content::Json;


mod db;
mod sensordata;
//mod posts;
pub mod schema;

use crate::sensordata::NewSensorData;

#[get("/hello")]
fn hello() -> String {

    println!("Hello, world!");
    "hello, world!!".to_string()
}

#[post("/sensor_data", format = "json", data = "<sensor_data>")]
fn post_sensor_data(sensor_data: String) -> String {
    println!("{}", sensor_data);
    "OK".to_string()
}

#[post("/test_post", format = "json", data = "<test_post_data>")]
fn test_post(test_post_data: String) -> String {
    test_post_data
}

fn main() {
    println!("Hello, world!");

    let mut rocket = rocket::ignite()
    .manage(db::init_pool());
    //rocket = users::mount(rocket);
    //rocket = posts::mount(rocket);
    rocket.mount("/api",
    routes![
        hello,
        post_sensor_data,
        test_post,
    ]).launch();
}
