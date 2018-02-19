#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::sync::Mutex;

use rocket::State;
use rocket_contrib::Json;

mod status;

type Cache = Mutex<status::Map>;

#[post("/", format = "application/json", data = "<message>")]
fn update(message: Json<status::TogglResponse>, map: State<Cache>) {
    let mut locked_map = map.lock().expect("can't lock the map");
    locked_map.insert(message.id.to_owned(), message.copy_data().unwrap());
}

#[get("/", format = "application/json")]
fn fetch(map: State<Cache>) -> Json<status::Map> {
    let v = map.lock().expect("can't lock the map").clone();
    Json(v)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![fetch, update])
        .manage(Cache::new(status::Map::new()))
        .launch();
}
