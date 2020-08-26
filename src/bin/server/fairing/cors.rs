use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Header, Method};
use rocket::{Request, Response};
use std::io::Cursor;

pub struct Cors;

#[async_trait::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

        if request.method() == Method::Options {
            response.set_header(ContentType::Plain);
            response.set_sized_body(0, Cursor::new(""));
        }
    }
}
