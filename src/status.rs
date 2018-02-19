
#[derive(Debug, Deserialize, Serialize)]
pub struct Status {
    pub start: String,
    pub description: String,
}

impl Clone for Status {
    fn clone(&self) -> Status {
        Status {
            start: self.start.clone(),
            description: self.description.clone(),
        }
    }
}

pub type ID = usize;

#[derive(Debug, Deserialize, Serialize)]
pub struct TogglResponse {
    #[serde(default)]
    pub id: ID,
    pub data: Option<Status>,
}

impl TogglResponse {
    pub fn copy_data(&self) -> Option<Status> {
        self.data.clone()
    }
}
