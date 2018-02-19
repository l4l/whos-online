extern crate reqwest;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate daemonize;
#[macro_use]
extern crate log;


mod status;
mod toggl;

use std::thread::sleep;
use std::env;

const USAGE: &'static str = "Usage: whosd <api_token> <user_id> [<http_host> <period>]";
const DEFAULT_HOST: &'static str = "http://127.0.0.1:8080";
const REPORT_PERIOD: u64 = 30;
const PID_FILE: &'static str = "/tmp/whosd.pid";

fn daemon_loop(token: &str, addr: &str, id: status::ID) {
    let mut res = toggl::check(token).expect("Ill-formed response");
    debug!("Info retrieved");

    res.id = id;
    let res = toggl::report(&res, addr);
    if res {
        info!("Info reported");
    } else {
        error!("Error in reporting");
    }
}

fn main() {
    let api_token = env::args().nth(1).expect(USAGE);
    let id = env::args().nth(2).expect(USAGE);
    let addr = env::args().nth(3).unwrap_or(DEFAULT_HOST.to_string());
    let period = std::time::Duration::from_secs(
        env::args()
            .nth(4)
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(REPORT_PERIOD),
    );

    match daemonize::Daemonize::new().pid_file(PID_FILE).start() {
        Ok(()) => {
            loop {
                daemon_loop(&api_token, &addr, id.to_owned());
                sleep(period);
            }
        }
        Err(e) => error!("{}", e),
    }
}
