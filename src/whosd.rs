use std::thread::sleep;
use std::time::Duration;
use daemonize::Daemonize;
use status;
use toggl;

const PID_FILE: &'static str = "/tmp/whosd.pid";

fn main_loop(token: &str, addr: &str, id: status::ID, wid: Option<i64>) {
    let mut res = toggl::check(token).expect("Ill-formed response");
    debug!("Info retrieved");

    let rwid = res.data.iter().next().and_then(|ref d| d.wid);
    if wid.is_some() && wid != rwid {
        return;
    }
    res.id = id;
    let res = toggl::report(&res, addr);
    if res {
        info!("Info reported");
    } else {
        error!("Error in reporting");
    }
}

pub fn launch(api_token: &str, addr: &str, id: status::ID, period: Duration, wid: Option<String>) {
    let wid = wid.and_then(|x| toggl::get_wid(api_token, &x));

    match Daemonize::new().pid_file(PID_FILE).start() {
        Ok(()) => {
            loop {
                main_loop(&api_token, &addr, id.to_owned(), wid);
                sleep(period);
            }
        }
        Err(e) => error!("{}", e),
    }
}
