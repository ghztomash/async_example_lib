pub mod error;
pub mod provider;
pub mod response;

#[cfg(feature = "sync")]
pub use synchr::perform_request;
#[cfg(feature = "async")]
pub use asynchr::perform_request;

#[synca::synca(
  #[cfg(feature = "async")]
  pub mod asynchr { },
  #[cfg(feature = "sync")]
pub mod synchr { 
    sync!();
    replace!(
      #[tokio::test] => #[test]
    );
  }
)]
mod request {

use crate::response::Response;
use crate::error::Error;
use crate::provider::{ServiceProvider,Service};

pub async fn perform_request() -> Result<Response, Error> {
    let provider = Service::new(ServiceProvider::HttpBin);
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
}
