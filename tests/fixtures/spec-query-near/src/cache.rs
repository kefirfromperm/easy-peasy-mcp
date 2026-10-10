pub async fn get(&self, key: &str) -> Value {
    let mut guard = self.inner.lock().unwrap();
    log::debug!("fetching {key}");
    let value = fetch(key).await?;
    guard.insert(key, value.clone());
    value
}
