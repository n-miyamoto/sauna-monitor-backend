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

#[get("/")]
fn hello_root() -> String {

    println!("Hello, world! root");
    "hello, world root!!".to_string()
}

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

//ambient compatible endpoint.
#[post("/v2/channels/<channel>/data", format = "json", data = "<sensor_data>")]
fn post_ambient_data(channel: u16, sensor_data: String) -> String {
    println!("channel : {}", channel);
    println!("{}", sensor_data);
    "OK".to_string()
}

fn main() {
    println!("Hello, world!");

    let mut rocket = rocket::ignite()
    .manage(db::init_pool());
    //rocket = users::mount(rocket);
    //rocket = posts::mount(rocket);
    rocket.mount("/api",
    routes![
        hello_root,
        hello,
        post_sensor_data,
        post_ambient_data,
    ]).launch();
}
