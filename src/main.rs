#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate docopt;
#[macro_use]
extern crate log;

mod whosb;
extern crate telebot;
extern crate tokio_core;
extern crate futures;

mod whosc;
extern crate rocket;
extern crate rocket_contrib;
extern crate lru_time_cache;

mod whosd;
extern crate daemonize;

mod status;
mod toggl;

const USAGE: &'static str = "
Whos-online is a set of tools for tracking online colleagues.

Usage:
  whos-online (-b | --bot) [--token=<bot_token>] [--host=<host>]
  whos-online (-c | --collector)
  whos-online (-d | --daemon) <token> <user> [--host=<host>] [--period=<period>] [--workspace=<ws>]
  whos-online (-h | --help)
  whos-online --version

Options:
  -b --bot                 Launch bot
  -c --collector           Launch tracking data collector
  -d --daemon              Launch submitting daemon
  --token=<bot_token>  Telegram bot token
  --host=<host>            Host for data reporting [default: http://127.0.0.1:8080]
  --period=<period>        Period of data reports [default: 30]
  --workspace=<ws>         Toggl workspace for monitoring
  --version                Show version.
  -h --help                Show this help.
";
const BOT_TOKEN: &'static str = "TG_BOT_TOKEN";
const BOT_MISSIG: &'static str = "Bot token should be properly set";

#[derive(Debug, Deserialize)]
struct Args {
    arg_token: Option<String>,
    arg_user: Option<String>,
    flag_token: Option<String>,
    flag_host: String,
    flag_period: u64,
    flag_workspace: Option<String>,
    // Launching types
    flag_bot: bool,
    flag_collector: bool,
    flag_daemon: bool,
}

fn main() {
    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    match (args.flag_bot, args.flag_collector, args.flag_daemon) {
        (true, _, _) => {
            let token = args.flag_token.or(std::env::var(BOT_TOKEN).ok()).expect(
                BOT_MISSIG,
            );
            let host = args.flag_host;
            whosb::launch(&token, &host);
        }
        (_, true, _) => {
            whosc::launch();
        }
        (_, _, true) => {
            let api_token = args.arg_token.expect(USAGE);
            let host = args.flag_host;
            let id = args.arg_user.expect(USAGE);
            whosd::launch(
                &api_token,
                &host,
                id,
                std::time::Duration::from_secs(args.flag_period),
                args.flag_workspace,
            );
        }
        _ => println!("{}", USAGE),
    }
}
