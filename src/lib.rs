pub mod error;
pub mod provider;
pub mod response;

use error::Error;
use response::Response;

pub async fn perform_request() -> Result<Response, Error> {
    let provider = provider::Service::new();
    provider.request().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let result = perform_request().await;
        assert!(result.is_ok());
    }
}
