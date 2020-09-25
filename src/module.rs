use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    pub identifier: String,
    pub name: String,
    pub live_streams: Vec<LiveStream>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiveStream {
    pub date: String,
    pub link: String,
}
