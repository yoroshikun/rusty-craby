use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserXEFile {
    pub last_updated: u64,
    pub users: Option<Vec<UserXEUser>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserXEUser {
    pub id: u64,
    pub default: String,
}
