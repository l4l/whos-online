#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate lru_time_cache;

use std::sync::Mutex;
use std::time::Duration;
use lru_time_cache::LruCache;

use rocket::State;
use rocket_contrib::Json;

mod status;

const OBSOLETE: u64 = 100;

type Cache = Mutex<LruCache<status::ID, Option<status::Status>>>;

#[post("/", format = "application/json", data = "<message>")]
fn update(message: Json<status::TrackingData>, cache: State<Cache>) {
    let mut map = cache.lock().expect("Can't lock the map at update");
    let ref id = message.id;
    map.insert(id.to_owned(), message.copy_data());
}

#[get("/", format = "application/json")]
fn fetch(cache: State<Cache>) -> Json<status::Map> {
    let mut map = cache.lock().expect("Can't lock the map at fetch");
    let v: status::Map = map.iter()
        .map(|(k, v)| (k.to_owned(), v.to_owned()))
        .collect();
    Json(v)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![fetch, update])
        .manage(Cache::new(
            LruCache::<status::ID, Option<status::Status>>::with_expiry_duration(
                Duration::from_secs(OBSOLETE),
            ),
        ))
        .launch();
}
