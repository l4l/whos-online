
#[derive(Debug, Deserialize, Serialize)]
pub struct Status {
    start: String,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TogglResponse {
    data: Option<Status>,
}
