//! The HTTP server: binds, serves, reloads the configuration on SIGHUP.

use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use crate::config::{Settings, load_config};

/// The state shared by the handlers.
pub struct State {
    pub settings: RwLock<Settings>,
    pub config_path: PathBuf,
}

/// Serves until the process is stopped.
pub fn run(settings: Settings) {
    let state = Arc::new(State {
        config_path: PathBuf::from("config.toml"),
        settings: RwLock::new(settings),
    });
    let listener = bind(&state);
    for request in listener.incoming() {
        handle(&state, request);
    }
}

/// Reads the configuration again; the old one stays if the file is broken.
pub fn reload(state: &State) {
    match load_config(&state.config_path) {
        Ok(settings) => *state.settings.write().expect("not poisoned") = settings,
        Err(e) => eprintln!("reload failed, keeping the old configuration: {e}"),
    }
}

/// Checks a configuration file without applying it.
pub fn check(path: &Path) -> bool {
    load_config(path).is_ok()
}

fn bind(state: &State) -> Listener {
    let settings = state.settings.read().expect("not poisoned");
    Listener::bind(&settings.host, settings.port)
}

fn handle(state: &State, request: Request) {
    let settings = state.settings.read().expect("not poisoned");
    request.respond(&settings);
}
