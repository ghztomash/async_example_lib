pub struct HttpBin;

use crate::{error::Error, provider::Provider, response::Response};

#[async_trait::async_trait]
impl Provider for HttpBin {
    fn get_endpoint(&self) -> String {
        "https://httpbin.org/json".to_string()
    }

    fn add_auth(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        request.header("User-Agent", "nil")
    }

    async fn make_api_request(&self) -> Result<String, Error> {
        let endpoint = format!("https://httpbin.org/json");
        let response = reqwest::get(endpoint).await;
        super::handle_response(response).await
    }

    fn parse_reply(&self, json: String) -> Result<Response, Error> {
        Ok(Response::new(json))
    }
}
