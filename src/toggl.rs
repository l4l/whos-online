use status::TogglResponse;
use reqwest::Client;

pub fn check(token: &str) -> Option<TogglResponse> {
    let client = Client::new();
    let mut res = client
        .get("https://toggl.com/api/v8/time_entries/current")
        .basic_auth(token, Some("api_token"))
        .send()
        .ok()?;
    res.json::<TogglResponse>().ok()
}

pub fn report(r: &TogglResponse, url: &str) -> bool {
    Client::new().post(url).json(&r).send().is_ok()
}
