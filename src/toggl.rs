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
    pub wid: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct TogglResponse {
    #[serde(default)]
    pub id: String,
    pub data: Option<TogglData>,
}

#[derive(Debug, Deserialize, Serialize)]
struct WorkspaceResponse {
    #[serde(default)]
    pub id: i64,
    pub name: String,
}

pub fn get_wid(token: &str, name: &str) -> Option<i64> {
    let client = Client::new();
    let mut res = client
        .get("https://toggl.com/api/v8/workspaces")
        .basic_auth(token, Some("api_token"))
        .send()
        .ok()?;
    let ws: Vec<WorkspaceResponse> = res.json().ok()?;
    ws.into_iter().find(|ref resp| &resp.name == name).map(
        |resp| {
            resp.id.to_owned()
        },
    )
}
