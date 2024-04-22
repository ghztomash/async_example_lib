pub mod httpbin;
pub mod mock;

mod client;

use crate::error::Error;
use crate::response::Response;

use client::{Client, RequestBuilder};
use reqwest::StatusCode;

pub trait Provider {
    fn get_endpoint(&self) -> String;

    fn parse_reply(&self, content: String) -> Result<Response, Error>;

    #[inline]
    fn get_client(&self) -> RequestBuilder {
        let client = Client::new().get(self.get_endpoint());
        self.add_auth(client)
    }

    #[inline]
    fn add_auth(&self, request: RequestBuilder) -> RequestBuilder {
        request
    }
}

pub enum ServiceProvider {
    HttpBin,
    Mock,
}

impl ServiceProvider {
    pub fn build(self) -> Box<dyn Provider> {
        match self {
            Self::HttpBin => Box::new(httpbin::HttpBin),
            Self::Mock => Box::new(mock::Mock),
        }
    }
}

pub struct Service {
    provider: Box<dyn Provider>,
}

impl Service {
    pub fn new(provider: ServiceProvider) -> Self {
        Self {
            provider: provider.build(),
        }
    }

    #[maybe_async::maybe_async]
    pub async fn request(&self) -> Result<Response, Error> {
        let response = self.make_api_request().await?;
        self.provider.parse_reply(response)
    }

    #[maybe_async::maybe_async]
    async fn make_api_request(&self) -> Result<String, Error> {
        let response = self.provider.get_client().send().await;
        handle_response(response).await
    }
}

#[maybe_async::maybe_async]
async fn handle_response(response: reqwest::Result<client::Response>) -> Result<String, Error> {
    match response {
        Ok(response) => match response.status() {
            StatusCode::OK => Ok(response.text().await.unwrap()),
            s => Err(Error::CriticalError(s.to_string())),
        },
        Err(e) => Err(Error::CriticalError(e.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[maybe_async::test(
        feature="blocking",
        async(not(feature="blocking"), tokio::test)
    )]
    async fn test_make_request() {
        let expected = "hello mock".to_string();
        let provider = Service::new(ServiceProvider::Mock);
        let response = provider.request().await.unwrap();
        assert_eq!(response.content, expected);
    }

    #[maybe_async::test(
        feature="blocking",
        async(not(feature="blocking"), tokio::test)
    )]
    async fn test_handle_response() {
        let response = client::get("https://httpbin.org/status/200").await;
        let body = handle_response(response).await;
        assert!(body.is_ok(), "Response is an error {:#?}", body);
    }

    #[maybe_async::test(
        feature="blocking",
        async(not(feature="blocking"), tokio::test)
    )]
    async fn test_handle_response_error() {
        let response = client::get("https://httpbin.org/status/500").await;
        let body = handle_response(response).await;
        assert!(body.is_err(), "Response should be an error {:#?}", body);
        let body = body.unwrap_err();
        assert_eq!(
            body.to_string(),
            "Critical error",
            "Wrong error {:#?}",
            body
        );
    }
}
