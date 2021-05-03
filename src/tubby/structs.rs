use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserTubbyFile {
    pub last_updated: u64,
    pub users: Option<Vec<UserTubbyUser>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserTubbyUser {
    pub name: String,
    pub expires: u64,
}
