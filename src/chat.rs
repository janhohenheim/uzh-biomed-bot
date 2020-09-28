use serde::{Deserialize, Serialize};
use tbot::types::chat::Id;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Chat {
    pub id: Id,
}
