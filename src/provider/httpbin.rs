use crate::{error::Error, provider::Provider, response::Response};
use reqwest::RequestBuilder;

pub struct HttpBin;

impl Provider for HttpBin {
    fn get_endpoint(&self) -> String {
        "https://httpbin.org/json".to_string()
    }

    fn add_auth(&self, request: RequestBuilder) -> RequestBuilder {
        request.header("User-Agent", "nil")
    }

    fn parse_reply(&self, json: String) -> Result<Response, Error> {
        Ok(Response::new(json))
    }
}
