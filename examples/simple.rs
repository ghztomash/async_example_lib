use std::error::Error;

#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let result = example_lib::perform_request().await?;

    println!("Hello world: {} ", result.content);
    Ok(())
}
#[cfg(feature = "sync")]
fn main() -> Result<(), Box<dyn Error>> {
    let result = example_lib::perform_request()?;

    println!("Hello world: {} ", result.content);
    Ok(())
}
