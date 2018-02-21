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

fn main() {
    let mut lp = Core::new().unwrap();
    let bot = bot::RcBot::new(lp.handle(), &env::var(BOT_TOKEN).unwrap()).update_interval(200);

    let handle = bot.new_cmd("/ask").and_then(|(bot, msg)| {
        let text = fetch(DEFAULT_HOST)
            .map(|m| {
                m.into_iter()
                    .map(|(k, v)| {
                        let data_prnt = v.map(|d| format!("online [{}]", d.description))
                            .unwrap_or("offline".to_string());
                        format!("{} is {}", k, data_prnt)
                    })
                    .collect()
            })
            .map(|m: String| if m.is_empty() {
                NO_USERS.to_string()
            } else {
                m
            })
            .unwrap_or(FETCH_ERROR.to_string());
        bot.message(msg.chat.id, text).send()
    });

    bot.register(handle);
    bot.run(&mut lp).unwrap();
}
