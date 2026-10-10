//! The service reads its settings from the environment.

fn main() {
    let env = std::env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());
    println!("starting in {env}");
}
