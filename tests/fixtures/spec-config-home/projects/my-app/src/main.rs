use std::env;

const VERSION: &str = "0.3.0";

fn main() {
    let name = env::args().nth(1).unwrap_or_default();
    println!("my-app {VERSION}: {name}");
}
