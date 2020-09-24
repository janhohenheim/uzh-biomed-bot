use serde::{Deserialize, Serialize};
use tbot::types::chat::Id;

#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    pub id: Id,
}
