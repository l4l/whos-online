use reqwest::Client;
use status::{TrackingData, Status};

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
    let data = TrackingData {
        id: r.id.to_owned(),
        data: r.data.iter().next().and_then(|ref s| {
            Some(Status {
                start: s.start.to_owned(),
                description: s.description.to_owned(),
            })
        }),
    };
    Client::new().post(url).json(&data).send().is_ok()
}

#[derive(Debug, Deserialize)]
pub struct TogglData {
    pub start: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct TogglResponse {
    #[serde(default)]
    pub id: String,
    pub data: Option<TogglData>,
}
