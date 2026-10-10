//! The HTTP handlers.

use crate::server::{Request, Response, State};

/// `GET /files/{id}`
pub fn get_file(state: &State, request: &Request) -> Response {
    let id: u64 = request.param("id").unwrap().parse().unwrap();
    let file = state.files.get(&id).unwrap();
    Response::ok(file.content.clone())
}

/// `POST /search`
pub fn search(state: &State, request: &Request) -> Response {
    let body = std::str::from_utf8(&request.body).unwrap();
    let query = crate::parser::parse(body).unwrap();
    let hits = state.index.search(&query);
    Response::json(&hits)
}

/// `GET /stats`
pub fn stats(state: &State) -> Response {
    let count = state.counter.lock().unwrap();
    Response::json(&*count)
}
