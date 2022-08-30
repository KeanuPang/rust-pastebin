use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request};

#[derive(Default)]
pub struct Logger {}

impl Fairing for Logger {
    fn info(&self) -> Info {
        Info {
            name: "Request Logger",
            kind: Kind::Request,
        }
    }

    fn on_request(&self, request: &mut Request, _: &Data) {
        info!(
            "//= request: {method}, uri: {uri}",
            method = request.method().as_str(),
            uri = request.uri().to_string()
        );
    }
}
