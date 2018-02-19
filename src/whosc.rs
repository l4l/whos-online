#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::sync::Mutex;
use std::collections::HashMap;

use rocket::State;
use rocket_contrib::Json;

mod status;

type Map = HashMap<status::ID, status::Status>;
type Cache = Mutex<Map>;

#[post("/", format = "application/json", data = "<message>")]
fn update(message: Json<status::TogglResponse>, map: State<Cache>) {
    let mut locked_map = map.lock().expect("can't lock the map");
    locked_map.insert(message.id, message.copy_data().unwrap());
}

#[get("/", format = "application/json")]
fn fetch(map: State<Cache>) -> Json<Map> {
    let v = map.lock().expect("can't lock the map").clone();
    Json(v)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![fetch, update])
        .manage(Cache::new(HashMap::new()))
        .launch();
}
