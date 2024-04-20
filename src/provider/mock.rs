pub struct Mock;

use crate::{error::Error, provider::Provider, response::Response};
use std::{thread, time};

#[async_trait::async_trait]
impl Provider for Mock {
    fn get_endpoint(&self) -> String {
        "https://httpbin.org/status/200".to_string()
    }

    async fn make_api_request(&self) -> Result<String, Error> {
        // simulate blocking api call
        thread::sleep(time::Duration::from_millis(100));
        Ok("hello mock".to_string())
    }

    fn parse_reply(&self, content: String) -> Result<Response, Error> {
        Ok(Response::new(content))
    }
}
