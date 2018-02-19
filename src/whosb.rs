extern crate telebot;
extern crate tokio_core;
extern crate futures;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;

use telebot::bot;
use tokio_core::reactor::Core;
use futures::stream::Stream;
use futures::Future;

// import all available functions
use telebot::functions::*;
use reqwest::Client;

use std::env;

mod status;

fn main() {
    let mut lp = Core::new().unwrap();
    let bot = bot::RcBot::new(lp.handle(), &env::var("TG_BOT_TOKEN").unwrap()).update_interval(200);

    let handle = bot.new_cmd("/ask").and_then(|(bot, msg)| {
        let text = reqwest::get("http://127.0.0.1:8080")
            .and_then(|mut x| x.text())
            .unwrap_or("nothing found".to_string());
        bot.message(msg.chat.id, text).send()
    });

    bot.register(handle);
    bot.run(&mut lp).unwrap();
}
