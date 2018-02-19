
#[derive(Debug, Deserialize, Serialize)]
pub struct Status {
    pub start: String,
    pub description: String,
}

pub type ID = usize;

#[derive(Debug, Deserialize, Serialize)]
pub struct TogglResponse {
    #[serde(default)]
    pub id: ID,
    pub data: Option<Status>,
}
