extern crate reqwest;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate daemonize;

extern crate docopt;

#[macro_use]
extern crate log;


mod status;
mod toggl;

use std::thread::sleep;
use docopt::Docopt;

const USAGE: &'static str = "
Whos-online daemon.

Usage:
  whosd <token> <user> [--host=<host>] [--period=<period>]
  whosd (-h | --help)
  whosd --version

Options:
  -h --help           Show this help.
  --version           Show version.
  --host=<host>       Host for data reporting [default: http://127.0.0.1:8080]
  --period=<period>   Period of data reports [default: 30]
  --workspace=<ws>    Toggl workspace for monitoring
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_token: Option<String>,
    arg_user: Option<String>,
    flag_host: String,
    flag_period: u64,
}

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
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let api_token = args.arg_token.expect(USAGE);
    let id = args.arg_user.expect(USAGE);
    let addr = args.flag_host;
    let period = std::time::Duration::from_secs(args.flag_period);

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
