pub async fn save(&self, value: Value) {
    let mut guard = self.inner.lock().unwrap();
    guard.push(value);
    drop(guard);
    self.saved += 1;
    self.log_save();
    self.notify();
    self.flush().await;
}
