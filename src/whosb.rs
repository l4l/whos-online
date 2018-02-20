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

fn main() {
    let mut lp = Core::new().unwrap();
    let bot = bot::RcBot::new(lp.handle(), &env::var("TG_BOT_TOKEN").unwrap()).update_interval(200);

    let handle = bot.new_cmd("/ask").and_then(|(bot, msg)| {
        let text = reqwest::get("http://127.0.0.1:8080")
            .and_then(|mut x| x.text())
            .ok()
            .and_then(|resp| serde_json::from_str(&resp).ok())
            .map(|m: status::Map| {
                m.into_iter()
                    .map(|(k, v)| {
                        let data_prnt = v.map(|d| format!("online [{}]", d.description))
                            .unwrap_or("offline".to_string());
                        format!("{} is {}", k, data_prnt)
                    })
                    .collect()
            })
            .unwrap_or("nothing found".to_string());
        bot.message(msg.chat.id, text).send()
    });

    bot.register(handle);
    bot.run(&mut lp).unwrap();
}
