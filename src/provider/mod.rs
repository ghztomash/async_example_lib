pub mod httpbin;
pub mod mock;

use crate::error::Error;
use crate::response::Response;

use reqwest::StatusCode;

#[async_trait::async_trait]
pub trait Provider {
    async fn make_api_request(&self) -> Result<String, Error>;
    fn parse_reply(&self, content: String) -> Result<Response, Error>;
}

pub struct Service {
    provider: Box<dyn Provider>,
}

impl Service {
    pub fn new() -> Self {
        Self {
            provider: Box::new(httpbin::HttpBin),
        }
    }

    pub async fn request(&self) -> Result<Response, Error> {
        let response = self.provider.make_api_request().await?;
        self.provider.parse_reply(response)
    }
}

async fn handle_response(response: reqwest::Result<reqwest::Response>) -> Result<String, Error> {
    match response {
        Ok(response) => match response.status() {
            StatusCode::OK => Ok(response.text().await.unwrap()),
            s => Err(Error::CriticalError(s.to_string())),
        },
        Err(e) => Err(Error::CriticalError(e.to_string())),
    }
}
