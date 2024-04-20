pub struct HttpBin;

use crate::{error::Error, provider::Provider, response::Response};

#[async_trait::async_trait]
impl Provider for HttpBin {
    async fn make_api_request(&self) -> Result<String, Error> {
        let endpoint = format!("https://www.iplocate.io/api/lookup");
        let response = reqwest::get(endpoint).await;
        super::handle_response(response).await
    }

    fn parse_reply(&self, json: String) -> Result<Response, Error> {
        Ok(Response::new(json.parse::<String>().unwrap()))
    }
}
