use tokio::sync::Mutex;

pub async fn refresh(cache: &Mutex<Vec<u8>>) {
    cache.lock().await.clear();
}
