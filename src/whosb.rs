use telebot::bot;
use telebot::functions::*;

use reqwest::get;
use serde_json::from_str;
use tokio_core::reactor::Core;
use futures::stream::Stream;

use status::Map;

const NO_USERS: &'static str = "No users found";
const FETCH_ERROR: &'static str = "Nothing found";

fn fetch(host: &str) -> Option<Map> {
    get(host).and_then(|mut x| x.text()).ok().and_then(|resp| {
        from_str(&resp).ok()
    })
}

fn print_all(map: Map) -> String {
    let s = map.into_iter()
        .map(|(k, v)| {
            let data_prnt = v.map(|d| format!("online [{}]", d.description)).unwrap_or(
                "offline".to_string(),
            );
            format!("{} is {}", k, data_prnt)
        })
        .collect::<String>();

    match s.is_empty() {
        true => NO_USERS.to_string(),
        _ => s,
    }
}

pub fn launch(token: &str, host: &str) {
    let mut lp = Core::new().unwrap();
    let host = host.to_string();
    let bot = bot::RcBot::new(lp.handle(), &token).update_interval(200);

    let handle = bot.new_cmd("/ask").and_then(move |(bot, msg)| {
        let text = fetch(&host).map(print_all).unwrap_or(
            FETCH_ERROR.to_string(),
        );
        bot.message(msg.chat.id, text).send()
    });

    bot.register(handle);
    bot.run(&mut lp).unwrap();
}
