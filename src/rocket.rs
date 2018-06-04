extern crate rocket;

use self::rocket::http::ContentType;
use self::rocket::request::Request;
use self::rocket::response::{self, Responder, Response};
use super::document::Document;
use std::io::Cursor;

#[cfg(feature = "rocket-support")]
impl<'r> Responder<'r> for Document {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(self.to_string()))
            .header(ContentType::new("image", "svg+xml"))
            .ok()
    }
}
