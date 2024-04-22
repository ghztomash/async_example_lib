use std::error::Error;

#[cfg_attr(not(feature = "blocking"), tokio::main)]
#[maybe_async::maybe_async]
async fn main() -> Result<(), Box<dyn Error>> {
    let result = example_lib::perform_request().await?;
    println!("Hello world: {} ", result.content);
    Ok(())
}
