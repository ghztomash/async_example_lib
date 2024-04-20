use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let result = example_lib::perform_request().await?;
    println!("Hello world: {} ", result.content);
    Ok(())
}
