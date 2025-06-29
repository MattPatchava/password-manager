use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Store {
    pub meta: Meta,
    pub entries: std::collections::HashMap<String, Entry>,
}

#[derive(Serialize, Deserialize)]
pub struct Meta {
    pub salt: String,
}

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub username: String,
    pub password: String,
    pub encrypted: bool,
    pub nonce: Option<String>,
}
