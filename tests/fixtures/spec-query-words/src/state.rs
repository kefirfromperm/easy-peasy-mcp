use std::sync::Mutex;

pub static STATE: Mutex<Vec<u8>> = Mutex::new(Vec::new());

pub async fn load() {}
