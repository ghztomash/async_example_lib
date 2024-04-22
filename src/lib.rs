pub mod error;
pub mod provider;
pub mod response;

use error::Error;
use provider::ServiceProvider;
use response::Response;

#[maybe_async::maybe_async]
pub async fn perform_request() -> Result<Response, Error> {
    let provider = provider::Service::new(ServiceProvider::HttpBin);
    provider.request().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[maybe_async::test(
        feature="blocking",
        async(not(feature="blocking"), tokio::test)
    )]
    async fn it_works() {
        let result = perform_request().await;
        assert!(result.is_ok());
    }
}
