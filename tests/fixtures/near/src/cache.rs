fn get(key: &str) -> Value {
let mut guard = self.inner.lock();
let cached = guard.get(key);
let value = fetch(key).await?;
value
}
