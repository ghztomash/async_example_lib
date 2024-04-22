use std::error::Error;

#[maybe_async::async_impl]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let result = example_lib::perform_request().await?;
    println!("Hello world: {} ", result.content);
    Ok(())
}

#[maybe_async::sync_impl]
fn main() -> Result<(), Box<dyn Error>> {
    let result = example_lib::perform_request()?;
    println!("Hello world: {} ", result.content);
    Ok(())
}

