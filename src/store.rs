use anyhow::Result;

use crate::hashing::password::generate_salt;
use crate::models::{Meta, Store};

pub fn load_store(file_path: &std::path::Path) -> Result<Store> {
    let store: Store = match std::fs::File::open(file_path) {
        Ok(file) => serde_json::from_reader(file)?,
        Err(_) => init_new_store()?,
    };

    Ok(store)
}

pub fn init_new_store() -> Result<Store> {
    let salt: String = generate_salt();

    let store: Store = Store {
        meta: { Meta { salt } },
        entries: std::collections::HashMap::new(),
    };

    Ok(store)
}
