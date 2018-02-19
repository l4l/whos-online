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

fn daemon_loop(token: &str, addr: &str) {
    let res = toggl::check(token).expect("Ill-formed response");
    info!(
        "{} in submitting",
        if toggl::report(&res, addr) {
            "success"
        } else {
            "error"
        }
    );
}

fn main() {
    let api_token = env::args().nth(1).expect(
        "Usage: whosd <api_token> [<http_host> <period>]",
    );
    let addr = env::args().nth(2).unwrap_or(
        "http://127.0.0.1:8080".to_string(),
    );
    let period = std::time::Duration::from_secs(
        env::args()
            .nth(3)
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(5),
    );

    match daemonize::Daemonize::new().pid_file("/tmp/whosd").start() {
        Ok(()) => {
            loop {
                daemon_loop(&api_token, &addr);
                sleep(period);
            }
        }
        Err(e) => error!("{}", e),
    }
}
