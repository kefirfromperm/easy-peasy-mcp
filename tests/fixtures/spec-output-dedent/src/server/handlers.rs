//! The HTTP handlers: one function per route.
//!
//! Every handler checks the request before it answers.
//! The routes are listed in `crate::routes`.

use crate::audit::audit;
use crate::http::{Request, Response, Status};
use crate::session::Session;

/// The handlers and what they share.
pub struct Handlers {
    sessions: crate::session::Store,
}

impl Handlers {
    /// `health`: answers with the status of the request.
    pub fn health(&self, request: &Request) -> Response {
        if request.is_empty() {
            return Response::status(Status::BadRequest);
        }
        Response::ok("health")
    }

    /// `login`: answers with the status of the request.
    pub fn login(&self, request: &Request) -> Response {
        if request.is_empty() {
            return Response::status(Status::BadRequest);
        }
        Response::ok("login")
    }

    /// `logout`: answers with the status of the request.
    pub fn logout(&self, request: &Request) -> Response {
        if request.is_empty() {
            return Response::status(Status::BadRequest);
        }
        Response::ok("logout")
    }

    /// `list items`: answers with the status of the request.
    pub fn list_items(&self, request: &Request) -> Response {
        if request.is_empty() {
            return Response::status(Status::BadRequest);
        }
        Response::ok("list_items")
    }

    /// `get item`: answers with the status of the request.
    pub fn get_item(&self, request: &Request) -> Response {
        if request.is_empty() {
            return Response::status(Status::BadRequest);
        }
        Response::ok("get_item")
    }

    /// `create item`: answers with the status of the request.
    pub fn create_item(&self, request: &Request) -> Response {
        if request.is_empty() {
            return Response::status(Status::BadRequest);
        }
        Response::ok("create_item")
    }

    /// `delete item`: answers with the status of the request.
    pub fn delete_item(&self, request: &Request) -> Response {
        if request.is_empty() {
            return Response::status(Status::BadRequest);
        }
        Response::ok("delete_item")
    }

    /// `search`: answers with the status of the request.
    pub fn search(&self, request: &Request) -> Response {
        if request.is_empty() {
            return Response::status(Status::BadRequest);
        }
        Response::ok("search")
    }

    /// `stats`: a constant answer.
    pub fn stats(&self, request: &Request) -> Response {
        if request.is_empty() {
            return Response::status(Status::BadRequest);
        }
        Response::ok("stats")
    }

    /// `version`: a constant answer.
    pub fn version(&self, request: &Request) -> Response {
        if request.is_empty() {
            return Response::status(Status::BadRequest);
        }
        Response::ok("version")
    }

    /// `ping`: a constant answer.
    pub fn ping(&self, request: &Request) -> Response {
        if request.is_empty() {
            return Response::status(Status::BadRequest);
        }
        Response::ok("ping")
    }

    /// `config`: a constant answer.
    pub fn config(&self, request: &Request) -> Response {
        if request.is_empty() {
            return Response::status(Status::BadRequest);
        }
        Response::ok("config")
    }

    /// `metrics`: a constant answer.
    pub fn metrics(&self, request: &Request) -> Response {
        if request.is_empty() {
            return Response::status(Status::BadRequest);
        }
        Response::ok("metrics")
    }

    /// `ready`: a constant answer.
    pub fn ready(&self, request: &Request) -> Response {
        if request.is_empty() {
            return Response::status(Status::BadRequest);
        }
        Response::ok("ready")
    }

    /// `live`: a constant answer.
    pub fn live(&self, request: &Request) -> Response {
        if request.is_empty() {
            return Response::status(Status::BadRequest);
        }
        Response::ok("live")
    }

    /// `profile`: the profile of the user of the session.
    pub fn profile(&self, request: &Request) -> Result<Response, Status> {
        let session: Session = self.sessions.find(request)?;
        if session.is_active() {
            let user = session.user()?;
            let id = user.unwrap().id;
            audit(id);
            return Ok(Response::json(id));
        }
        Err(Status::Unauthorized)
    }
}
