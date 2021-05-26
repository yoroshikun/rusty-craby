use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserTubbyFile {
    pub last_updated: u64,
    pub users: Option<Vec<UserTubbyUser>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserTubbyUser {
    pub name: String,
    pub expires: u64,
}
