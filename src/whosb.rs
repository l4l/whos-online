extern crate telebot;
extern crate tokio_core;
extern crate futures;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use telebot::bot;
use tokio_core::reactor::Core;
use futures::stream::Stream;

// import all available functions
use telebot::functions::*;

use std::env;

mod status;

const USAGE: &'static str = "Usage: whosb [<whosc_host>]\n\nNote: env TG_BOT_TOKEN should be properly set";
const BOT_TOKEN: &'static str = "TG_BOT_TOKEN";
const DEFAULT_HOST: &'static str = "http://127.0.0.1:8080";
const NO_USERS: &'static str = "No users found";
const FETCH_ERROR: &'static str = "Nothing found";

fn fetch(host: &str) -> Option<status::Map> {
    reqwest::get(host)
        .and_then(|mut x| x.text())
        .ok()
        .and_then(|resp| serde_json::from_str(&resp).ok())
}

fn print_all(map: status::Map) -> String {
    let s = map.into_iter().map(|(k, v)| {
        let data_prnt = v.map(|d| format!("online [{}]", d.description))
            .unwrap_or("offline".to_string());
        format!("{} is {}", k, data_prnt)
    }).collect::<String>();

    match s.is_empty() {
        true => NO_USERS.to_string(),
        _ => s,
    }
}

fn main() {
    let mut lp = Core::new().unwrap();
    let host = env::args().nth(1).unwrap_or(DEFAULT_HOST.to_string());
    let bot = bot::RcBot::new(lp.handle(), &env::var(BOT_TOKEN).unwrap()).update_interval(200);

    let handle = bot.new_cmd("/ask").and_then(move |(bot, msg)| {
        let text = fetch(&host)
            .map(print_all)
            .unwrap_or(FETCH_ERROR.to_string());
        bot.message(msg.chat.id, text).send()
    });

    bot.register(handle);
    bot.run(&mut lp).unwrap();
}
