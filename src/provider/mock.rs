use crate::{error::Error, provider::Provider, response::Response};

pub struct Mock;

impl Provider for Mock {
    fn get_endpoint(&self) -> String {
        "https://httpbin.org/status/200".to_string()
    }

    fn parse_reply(&self, _content: String) -> Result<Response, Error> {
        Ok(Response::new("hello mock".to_string()))
    }
}
