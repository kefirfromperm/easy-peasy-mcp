// SPDX-License-Identifier: MIT
use app::config_loader;

fn main() {
    let config = config_loader::load("app.toml").expect("app.toml is readable");
    println!("{} bytes of configuration", config.len());
}
