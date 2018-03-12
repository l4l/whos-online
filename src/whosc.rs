use std::sync::Mutex;
use std::time::Duration;
use lru_time_cache::LruCache;

use rocket;
use rocket_contrib::Json;

use status::*;

const OBSOLETE: u64 = 100;

type Cache = Mutex<LruCache<ID, Option<Status>>>;

#[post("/", format = "application/json", data = "<message>")]
fn update(message: Json<TrackingData>, cache: rocket::State<Cache>) {
    let mut map = cache.lock().expect("Can't lock the map at update");
    let ref id = message.id;
    map.insert(id.to_owned(), message.copy_data());
}

#[get("/", format = "application/json")]
fn fetch(cache: rocket::State<Cache>) -> Json<Map> {
    let mut map = cache.lock().expect("Can't lock the map at fetch");
    let v: Map = map.iter()
        .map(|(k, v)| (k.to_owned(), v.to_owned()))
        .collect();
    Json(v)
}

pub fn launch() {
    rocket::ignite()
        .mount("/", routes![fetch, update])
        .manage(Cache::new(
            LruCache::<ID, Option<Status>>::with_expiry_duration(
                Duration::from_secs(OBSOLETE),
            ),
        ))
        .launch();
}
