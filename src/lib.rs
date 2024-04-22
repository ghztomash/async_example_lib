#![warn(missing_docs)]

//! # roux.rs
//! This crate provides simple access to the Reddit API.
//!
//! ## Using OAuth
//! To create an OAuth client with the reddit API, use the `Reddit` class.
//! ```rust
//! # use std::error::Error;
//!
//! # #[cfg_attr(not(feature = "blocking"), tokio::main)]
//! # #[maybe_async::maybe_async]
//! # async fn main() -> Result<(), Box<dyn Error>> {
//! let result = example_lib::perform_request().await?;
//! println!("Hello world: {} ", result.content);
//! # Ok(())
//! # }
//! ```
//!

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
