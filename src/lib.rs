pub mod error;
pub mod provider;
pub mod response;

use error::Error;
use provider::ServiceProvider;
use response::Response;

#[cfg(feature = "async")]
pub async fn perform_request() -> Result<Response, Error> {
    let provider = provider::Service::new(ServiceProvider::HttpBin);
    provider.request().await
}

#[cfg(feature = "sync")]
pub fn perform_request() -> Result<Response, Error> {
    let provider = provider::Service::new(ServiceProvider::HttpBin);
    provider.request()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn it_works() {
        let result = perform_request().await;
        assert!(result.is_ok());
    }

    #[cfg(feature = "sync")]
    #[test]
    fn it_works() {
        let result = perform_request();
        assert!(result.is_ok());
    }
}
