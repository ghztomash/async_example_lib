use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let result = example_lib::perform_request()?;
    println!("Hello world: {} ", result.content);
    Ok(())
}
