#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use std::sync::Mutex;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Instant;

use rocket::State;
use rocket_contrib::Json;

mod status;

const OBSOLETE: u64 = 100;

#[derive(Eq)]
struct Update {
    appear: Instant,
    id: status::ID,
}

impl Update {
    fn new(id: status::ID) -> Update {
        Update {
            appear: Instant::now(),
            id: id,
        }
    }
}

impl Ord for Update {
    fn cmp(&self, other: &Update) -> Ordering {
        self.appear.cmp(&other.appear)
    }
}

impl PartialOrd for Update {
    fn partial_cmp(&self, other: &Update) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Update {
    fn eq(&self, other: &Update) -> bool {
        self.appear == other.appear
    }
}

type Cache = Mutex<status::Map>;
type Recency = Mutex<BinaryHeap<Update>>;

#[post("/", format = "application/json", data = "<message>")]
fn update(message: Json<status::TogglResponse>, cache: State<Cache>, recency: State<Recency>) {
    let mut map = cache.lock().expect("can't lock the map");
    let mut rec = recency.lock().expect("can't lock the recency");
    let ref id = message.id;
    map.insert(id.to_owned(), message.copy_data().unwrap());
    rec.push(Update::new(id.to_owned()));
}

#[get("/", format = "application/json")]
fn fetch(cache: State<Cache>, recency: State<Recency>) -> Json<status::Map> {
    let mut map = cache.lock().expect("can't lock the map");
    let mut rec = recency.lock().expect("can't lock the recency");
    for i in rec.drain().rev().take_while(|x| {
        x.appear.elapsed().as_secs() >= OBSOLETE
    })
    {
        map.remove(&i.id);
    }
    let v = map.clone();
    Json(v)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![fetch, update])
        .manage(Cache::new(status::Map::new()))
        .manage(Recency::new(BinaryHeap::new()))
        .launch();
}
